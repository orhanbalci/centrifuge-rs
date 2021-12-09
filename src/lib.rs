pub mod client;
pub mod error;
pub mod protocol;
pub mod transport;

// use std::{thread, time::Duration};

// use prost::Message;
// use protocol::HistoryRequest;

// use crate::{
//     client::Client,
//     protocol::{command::MethodType, Command},
// };

// fn main() {
//     let mut c = Client::new("ws://localhost:8000/connection/websocket?format=protobuf");

//     //TODO this is not a valid connect command update this
//     // also this should be automatically send on a successfull connection
//     let mut cmd = Command {
//         id: 1,
//         method: 0, //MethodType::Connect,
//         params: Vec::new(),
//     };
//     cmd.set_method(MethodType::Connect);
//     // c.send(cmd);
//     loop {
//         dbg!(c.get_one());
//         thread::sleep(Duration::from_secs(3));
//     }
// }
