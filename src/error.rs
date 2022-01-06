use std::sync::mpsc::SendError;

use prost::{DecodeError, EncodeError};
use thiserror::Error;

pub type CentrifugeResult<T> = Result<T, CentrifugeError>;

#[derive(Error, Debug)]
pub enum CentrifugeError {
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
    #[error("Receive channel not readey")]
    ReceiveChannelNotReady,
    #[error("Transmit channel not readey")]
    TransmitChannelNotReady,
    #[error("Can not parse http header")]
    HttpHeaderError,
    #[error("Websocket error")]
    WebsocketError {
        #[from]
        source: async_tungstenite::tungstenite::Error,
    },
}
