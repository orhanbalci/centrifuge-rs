use std::{os::unix::prelude::ExitStatusExt, time::Duration};

pub mod client;
pub mod error;
pub mod protocol;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    let mut c =
        client::Client::new("ws://127.0.0.1:8000/connection/websocket?format=protobuf".into());
    c.set_name("orhans client".to_string());
    c.set_version("1.0".to_string());
    c.set_token("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM3MjIiLCJleHAiOjE2NDAyNDEwMjh9.FU_xm65IX01OEOjuCqW1R6e6HEdCtOzt_BnobuzTSCo".to_string());
    if let Ok(true) = c.connect().await {
        c.send_connect().await;
        loop {
            println!("waiting!");
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    } else {
        println!("can not connect to server");
    }
}
