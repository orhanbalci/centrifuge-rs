use crate::error::CentrifugeError;

use super::{
    error::CentrifugeResult,
    protocol::{Command, Reply},
};
use async_tungstenite::tokio::connect_async;
use async_tungstenite::tungstenite::protocol::Message;
use futures::AsyncReadExt;
use http::Request;
use prost::Message as PMessage;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};



pub trait Transport<R, W> {
    fn read(&self) -> CentrifugeResult<R>;
    fn write(&mut self, cmd: Command) -> CentrifugeResult<W>;
    fn close(&mut self) -> CentrifugeResult<W>;
}

pub struct WebsocketTransport {
    url: String,
    protocol_type: ProtocolType,
    request_tx: Option<Sender<Message>>,
    reply_rc: Option<Receiver<Message>>,
    closed: bool,
}

impl WebsocketTransport {
    pub fn new(url: String, protocol_type: ProtocolType) -> Self {
        WebsocketTransport {
            url,
            protocol_type,
            request_tx: None,
            reply_rc: None,
            closed: true,
        }
    }
}

impl Transport<Reply, ()> for WebsocketTransport {
    fn read(&self) -> CentrifugeResult<Reply> {
        if let Some(rc) = &self.reply_rc {
            if let Some(next_reply) = rc.iter().next() {
                match next_reply {
                    Message::Text(txt) => todo!(),
                    Message::Binary(bin) => {
                        crate::protocol::Reply::decode_length_delimited(&bin[..])
                            .map_err(|err| err.into())
                    }
                    Message::Close(_) => Err(CentrifugeError::SocketClosed),
                    Message::Ping(_) => Err(CentrifugeError::SocketClosed),
                    Message::Pong(_) => Err(CentrifugeError::SocketClosed),
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
                    .send(Message::Text(serde_json::to_string(&cmd).unwrap()))
                    .map_err(|err| err.into()),
                ProtocolType::Protobuf => {
                    let mut buffer: Vec<u8> = Vec::with_capacity(50);
                    cmd.encode_length_delimited(&mut buffer)?;
                    return tx.send(Message::Binary(buffer)).map_err(|err| err.into());
                }
            }
        } else {
            Err(CentrifugeError::TransmitChannelNotReady)
        }
    }

    fn close(&mut self) -> CentrifugeResult<()> {
        //TODO handle thread joins gracefully
        self.closed = true;
        if let Some(tx) = &self.request_tx {
            tx.send(Message::Close(None)).map_err(|err| err.into())
        } else {
            Err(CentrifugeError::TransmitChannelNotReady)
        }
    }
}

impl WebsocketTransport {
    pub async fn connect(&mut self, headers: &http::HeaderMap) -> CentrifugeResult<bool> {
        if (!self.is_closed()) {
            return Ok(false);
        }

        let reader = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
            })
        };
        Ok(true)
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }
}
