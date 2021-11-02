use super::protocol::{Command, Reply};
use anyhow::Result;
use prost::Message as PMessage;
use std::sync::mpsc::{Receiver, Sender};
use std::{net::TcpStream, sync::mpsc::channel};
use thiserror::Error;
use tungstenite::Message as TMessage;
use tungstenite::{
    client::IntoClientRequest, connect, stream::MaybeTlsStream, Error as TError, WebSocket,
};

use prost::{DecodeError, EncodeError};

type CResult<T> = Result<T, TransportError>;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Socket closed")]
    SocketClosed,
    #[error("Reply Not Ready")]
    ReplyNotReady,
    #[error("Websocket Error")]
    WebsocketError {
        #[from]
        source: TError,
    },
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
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    reply_tx: Sender<Reply>,
    reply_rc: Receiver<Reply>,
    disconnect_rc: Receiver<bool>,
    closed: bool,
}

impl WebsocketTransport {
    pub fn new<T>(
        url: T,
        protocol_type: ProtocolType,
        disconnect_rc: Receiver<bool>,
    ) -> CResult<Self>
    where
        T: IntoClientRequest,
    {
        let (socket, _) = connect(url)?;
        let (reply_tx, reply_rc) = channel();
        Ok(WebsocketTransport {
            protocol_type,
            socket,
            reply_tx,
            reply_rc,
            disconnect_rc,
            closed: false,
        })
    }
}

impl Transport<Reply, ()> for WebsocketTransport {
    fn read(&self) -> CResult<Reply> {
        if let Some(next_reply) = self.reply_rc.iter().next() {
            Ok(next_reply)
        } else if self.closed {
            Err(TransportError::SocketClosed)
        } else {
            Err(TransportError::ReplyNotReady)
        }
    }

    fn write(&mut self, cmd: Command) -> CResult<()> {
        match self.protocol_type {
            ProtocolType::Json => self
                .socket
                .write_message(TMessage::Text(serde_json::to_string(&cmd).unwrap()))
                .map_err(|err| err.into()),
            ProtocolType::Protobuf => {
                let mut buffer: Vec<u8> = Vec::with_capacity(50);
                cmd.encode_length_delimited(&mut buffer)?;
                return self
                    .socket
                    .write_message(TMessage::Binary(buffer))
                    .map_err(|err| err.into());
            }
        }
    }

    fn close(&mut self) -> CResult<()> {
        self.socket.close(None).map_err(|err| err.into())
    }
}
