use super::protocol::{Command, Reply};
use anyhow::Result;
use prost::Message as PMessage;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, SendError, Sender};
use std::thread::{self, JoinHandle};
use thiserror::Error;
use websocket::{ClientBuilder, OwnedMessage, WebSocketError};

use prost::{DecodeError, EncodeError};

type CResult<T> = Result<T, TransportError>;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Socket closed")]
    SocketClosed,
    #[error("Reply Not Ready")]
    ReplyNotReady,
    #[error("Protobuf Decode Error")]
    ProtobufDecodeError {
        #[from]
        source: DecodeError,
    },
    #[error("Protobuf Encode Error")]
    ProtobufEncodeError {
        #[from]
        source: EncodeError,
    },
    #[error("MessageQueue Error")]
    MessageQueueError {
        #[from]
        source: SendError<OwnedMessage>,
    },
    #[error("WebSocket Error")]
    WebSocketError {
        #[from]
        source: WebSocketError,
    },
}

pub enum ProtocolType {
    Json,
    Protobuf,
}

pub trait Transport<R, W> {
    fn read(&self) -> CResult<R>;
    fn write(&mut self, cmd: Command) -> CResult<W>;
    fn close(&mut self) -> CResult<W>;
}

pub struct WebsocketTransport {
    protocol_type: ProtocolType,
    request_tx: Sender<OwnedMessage>,
    reply_rc: Receiver<OwnedMessage>,
    // disconnect_rc: Receiver<bool>,
    sender_loop: JoinHandle<()>,
    receiver_loop: JoinHandle<()>,
    closed: bool,
}

impl WebsocketTransport {
    pub fn new(
        url: &str,
        protocol_type: ProtocolType,
        // disconnect_rc: Receiver<bool>,
    ) -> CResult<Self> {
        let client = ClientBuilder::new(url)
            .unwrap()
            .add_protocol("rust-websocket")
            .connect_insecure()?;

        println!("Successfully connected");

        let (mut receiver, mut sender) = client.split().unwrap();

        let (request_tx, rx) = channel();
        let (reply_tx, reply_rc) = channel();

        let tx_1 = request_tx.clone();

        let send_loop = thread::spawn(move || {
            loop {
                // Send loop
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = sender.send_message(&message);
                        // If it's a close message, just send it and then return.
                        return;
                    }
                    _ => (),
                }
                // Send the message
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&OwnedMessage::Close(None));
                        return;
                    }
                }
            }
        });

        let receive_loop = thread::spawn(move || {
            // Receive loop
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                };
                match message {
                    OwnedMessage::Close(_) => {
                        // Got a close message, so send a close message and return
                        let _ = tx_1.send(OwnedMessage::Close(None));
                        return;
                    }
                    OwnedMessage::Ping(data) => {
                        match tx_1.send(OwnedMessage::Pong(data)) {
                            // Send a pong in response
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                return;
                            }
                        }
                    }
                    // Say what we received
                    _ => {
                        println!("Receive Loop: {:?}", message);
                        let _ = reply_tx.send(message);
                    }
                }
            }
        });

        Ok(WebsocketTransport {
            protocol_type,
            request_tx,
            reply_rc,
            sender_loop: send_loop,
            receiver_loop: receive_loop,
            closed: false,
        })
    }
}

impl Transport<Reply, ()> for WebsocketTransport {
    fn read(&self) -> CResult<Reply> {
        if let Some(next_reply) = self.reply_rc.iter().next() {
            match next_reply {
                OwnedMessage::Text(txt) => todo!(),
                OwnedMessage::Binary(bin) => {
                    crate::protocol::Reply::decode_length_delimited(&bin[..])
                        .map_err(|err| err.into())
                }
                OwnedMessage::Close(_) => Err(TransportError::SocketClosed),
                OwnedMessage::Ping(_) => Err(TransportError::SocketClosed),
                OwnedMessage::Pong(_) => Err(TransportError::SocketClosed),
            }
        } else if self.closed {
            Err(TransportError::SocketClosed)
        } else {
            Err(TransportError::ReplyNotReady)
        }
    }

    fn write(&mut self, cmd: Command) -> CResult<()> {
        match self.protocol_type {
            ProtocolType::Json => self
                .request_tx
                .send(OwnedMessage::Text(serde_json::to_string(&cmd).unwrap()))
                .map_err(|err| err.into()),
            ProtocolType::Protobuf => {
                let mut buffer: Vec<u8> = Vec::with_capacity(50);
                cmd.encode_length_delimited(&mut buffer)?;
                return self
                    .request_tx
                    .send(OwnedMessage::Binary(buffer))
                    .map_err(|err| err.into());
            }
        }
    }

    fn close(&mut self) -> CResult<()> {
        //TODO handle thread joins gracefully
        self.request_tx
            .send(OwnedMessage::Close(None))
            .map_err(|err| err.into())
    }
}
