use std::collections::HashMap;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use crate::error::CentrifugeResult;
use crate::protocol::Reply;
use crate::transport::ProtocolType;

use crate::protocol::Command;
use crate::transport::Transport;
use crate::transport::WebsocketTransport;
use httparse::Header;

pub struct Client<T> {
    transport: T,
    message_id: AtomicU32,
    token: String,
    connect_data: String,
    headers: Vec<Header<'static>>,
    connecting: bool,
}

impl Client<WebsocketTransport> {
    pub fn new(url: &str) -> Self {
        Client {
            transport: WebsocketTransport::new(url.to_string(), ProtocolType::Protobuf),
            message_id: AtomicU32::new(0),
            token: String::new(),
            connect_data: String::new(),
            headers: Vec::new(),
            connecting: false,
        }
    }

    pub fn send(&mut self, cmd: Command) {
        self.transport.write(cmd);
    }

    pub fn get_one(&mut self) -> Option<Reply> {
        match self.transport.read() {
            Ok(r) => Some(r),
            Err(_) => {
                println!("reply not ready");
                None
            }
        }
    }

    pub fn get_message_id(&self) -> u32 {
        self.message_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn set_connect_data(&mut self, connect_data: String) {
        self.connect_data = connect_data;
    }

    pub fn set_header(&mut self, name: &'static str, value: &'static [u8]) {
        self.headers.push(Header { name, value });
    }

    // pub fn connect(&mut self) -> CentrifugeResult<bool> {
    //     self.connecting = true;
    //     self.transport
    //         .connect(&self.headers)
    //         .and_then(|connected| {})
    // }
}
