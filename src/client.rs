use futures::channel::mpsc::UnboundedSender;
use futures::{SinkExt, StreamExt};
use http::header::HeaderName;
use http::{HeaderMap, HeaderValue};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use crate::error::CentrifugeResult;
use async_tungstenite::tokio::connect_async;

use crate::protocol::{command::MethodType, Command, Reply};
use async_tungstenite::tungstenite::Message as TMessage;
use prost::Message;

lazy_static! {
    static ref CALLBACK_FUTURES: Arc<
        Mutex<
            HashMap<u32, fn(&mut Client, Reply) -> Pin<Box<dyn Future<Output = ()> + '_ + Send>>>,
        >,
    > = {
        let mut m = HashMap::new();
        Arc::new(Mutex::new(m))
    };
}

pub enum ProtocolType {
    Json,
    Protobuf,
}

pub struct Client {
    message_id: AtomicU32,
    token: String,
    connect_data: String,
    headers: HeaderMap,
    connecting: bool,
    name: String,
    version: String,
    // runtime: tokio::runtime::Runtime,
    message_transmitter: Option<UnboundedSender<Command>>,
    url: String,
}

impl Client {
    pub fn new(url: String) -> Self {
        Client {
            message_id: AtomicU32::new(1),
            token: String::new(),
            connect_data: String::new(),
            headers: HeaderMap::new(),
            connecting: false,
            name: String::new(),
            version: String::new(),
            // runtime: tokio::runtime::Builder::new_multi_thread()
            //     .enable_all()
            //     .build()
            //     .unwrap(),
            message_transmitter: None,
            url: url,
        }
    }

    pub async fn send(&mut self, cmd: Command) {
        //self.message_transmitter.as_ref().map(|mut tx|{ tx.send(cmd).await;});
        self.message_transmitter
            .as_ref()
            .clone()
            .unwrap()
            .send(cmd)
            .await;
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

    pub fn set_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.headers.append(name, value);
    }

    pub async fn connect(&mut self) -> CentrifugeResult<bool> {
        self.connecting = true;
        let mut rb = http::Request::builder();
        for (hname, hvalue) in self.headers.iter() {
            rb = rb.header(hname, hvalue);
        }
        let request = rb.uri(&self.url).body(());
        let (package_tx, package_rx) = futures::channel::mpsc::unbounded();
        self.message_transmitter = Some(package_tx);
        let (mut ws_stream, _) = connect_async(request.unwrap()).await?;
        let (write, read) = ws_stream.split();
        let reader = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                println!("message received {:?}", data);
            })
        };
        tokio::spawn(async {
            package_rx
                .map(|package| {
                    println!("Sending packet {:?}", package);
                    let mut buffer: Vec<u8> = Vec::with_capacity(50);
                    package
                        .encode_length_delimited(&mut buffer)
                        .expect("Failed to encode message");
                    Ok(TMessage::Binary(buffer))
                })
                .forward(write)
                .await
        });
        tokio::spawn(async { reader.await });
        Ok(true)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub async fn send_connect(&mut self) -> CentrifugeResult<()> {
        let connect_request: super::protocol::ConnectRequest = super::protocol::ConnectRequest {
            token: self.token.clone(),
            data: self.connect_data.as_bytes().to_vec().clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            subs: HashMap::new(),
        };

        let connect_request_encoded = connect_request.encode_to_vec();

        let mut cmd = Command {
            id: self.get_message_id(),
            method: 0, //MethodType::Connect,
            params: connect_request_encoded,
        };
        cmd.set_method(MethodType::Connect);
        if let Ok(mut callback_futures) = CALLBACK_FUTURES.lock() {
            callback_futures.insert(cmd.id, handle_connect_reply_async);
        } else {
            println!("can not get callback lock");
        }

        self.send(cmd).await;
        Ok(())
    }

    pub async fn send_request(&mut self, data: Vec<u8>) -> CentrifugeResult<()> {
        let send_request: super::protocol::SendRequest =
            super::protocol::SendRequest { data: data };

        let connect_request_encoded = send_request.encode_to_vec();
        let mut cmd = Command {
            id: self.get_message_id(),
            method: 0,
            params: connect_request_encoded,
        };
        cmd.set_method(MethodType::Send);
        self.send(cmd).await;
        Ok(())
    }
}

async fn handle_connect_reply(c: &mut Client, reply: Reply) {
    println!("received send reply")
}

async fn read_one_reply(c: &mut Client) -> Option<Reply> {
    //c.get_one().await
    None
}

fn handle_connect_reply_async(
    c: &mut Client,
    reply: Reply,
) -> Pin<Box<dyn Future<Output = ()> + '_ + Send>> {
    Box::pin(handle_connect_reply(c, reply))
}
