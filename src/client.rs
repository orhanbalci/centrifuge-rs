use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use crate::protocol::Reply;
use crate::transport::ProtocolType;

use crate::protocol::Command;
use crate::transport::Transport;
use crate::transport::WebsocketTransport;

pub struct Client<T> {
    transport: T,
    message_id: AtomicU32,
}

impl Client<WebsocketTransport> {
    pub fn new(url: &str) -> Self {
        Client {
            transport: WebsocketTransport::new(url, ProtocolType::Protobuf)
                .expect("Can not connect to server endpoint"),
            message_id: AtomicU32::new(0),
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
}
