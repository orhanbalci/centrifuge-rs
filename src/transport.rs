use crate::error::CentrifugeError;

use super::{
    error::CentrifugeResult,
    protocol::{Command, Reply},
};
use httparse::Header;
use prost::Message as PMessage;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self};
use websocket::{header::Headers, ClientBuilder, OwnedMessage};
pub enum ProtocolType {
    Json,
    Protobuf,
}

pub trait Transport<R, W> {
    fn read(&self) -> CentrifugeResult<R>;
    fn write(&mut self, cmd: Command) -> CentrifugeResult<W>;
    fn close(&mut self) -> CentrifugeResult<W>;
}

pub struct WebsocketTransport {
    url: String,
    protocol_type: ProtocolType,
    request_tx: Option<Sender<OwnedMessage>>,
    reply_rc: Option<Receiver<OwnedMessage>>,
    closed: bool,
}

impl WebsocketTransport {
    pub fn new(url: String, protocol_type: ProtocolType) -> Self {
        WebsocketTransport {
            url,
            protocol_type,
            request_tx: None,
            reply_rc: None,
            closed: false,
        }
    }
}

impl Transport<Reply, ()> for WebsocketTransport {
    fn read(&self) -> CentrifugeResult<Reply> {
        if let Some(rc) = &self.reply_rc {
            if let Some(next_reply) = rc.iter().next() {
                match next_reply {
                    OwnedMessage::Text(txt) => todo!(),
                    OwnedMessage::Binary(bin) => {
                        crate::protocol::Reply::decode_length_delimited(&bin[..])
                            .map_err(|err| err.into())
                    }
                    OwnedMessage::Close(_) => Err(CentrifugeError::SocketClosed),
                    OwnedMessage::Ping(_) => Err(CentrifugeError::SocketClosed),
                    OwnedMessage::Pong(_) => Err(CentrifugeError::SocketClosed),
                }
            } else if self.closed {
                Err(CentrifugeError::SocketClosed)
            } else {
                Err(CentrifugeError::ReplyNotReady)
            }
        } else {
            Err(CentrifugeError::ReceiveChannelNotReady)
        }
    }

    fn write(&mut self, cmd: Command) -> CentrifugeResult<()> {
        if let Some(tx) = &self.request_tx {
            match self.protocol_type {
                ProtocolType::Json => tx
                    .send(OwnedMessage::Text(serde_json::to_string(&cmd).unwrap()))
                    .map_err(|err| err.into()),
                ProtocolType::Protobuf => {
                    let mut buffer: Vec<u8> = Vec::with_capacity(50);
                    cmd.encode_length_delimited(&mut buffer)?;
                    return tx
                        .send(OwnedMessage::Binary(buffer))
                        .map_err(|err| err.into());
                }
            }
        } else {
            Err(CentrifugeError::TransmitChannelNotReady)
        }
    }

    fn close(&mut self) -> CentrifugeResult<()> {
        //TODO handle thread joins gracefully
        if let Some(tx) = &self.request_tx {
            tx.send(OwnedMessage::Close(None)).map_err(|err| err.into())
        } else {
            Err(CentrifugeError::TransmitChannelNotReady)
        }
    }
}

impl WebsocketTransport {
    pub fn connect(&mut self, headers: &Vec<Header>) -> CentrifugeResult<bool> {
        let headers =
            Headers::from_raw(&headers[..]).map_err(|_| CentrifugeError::HttpHeaderError)?;
        let client = ClientBuilder::new(&self.url)
            .unwrap()
            .add_protocol("rust-websocket")
            .custom_headers(&headers)
            .connect_insecure()?;

        let (mut receiver, mut sender) = client.split().unwrap();

        let (request_tx, rx) = channel();
        let (reply_tx, reply_rc) = channel();

        let tx_1 = request_tx.clone();
        self.request_tx = Some(request_tx);
        self.reply_rc = Some(reply_rc);

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

        // if client is created connection is ok
        return Ok(true);
    }
}
