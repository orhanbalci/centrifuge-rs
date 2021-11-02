pub mod protocol;
pub mod transport;
use prost::Message;
use protocol::HistoryRequest;
fn main() {
    let hr = HistoryRequest {
        channel: "MyChannel".to_owned(),
        limit: 10,
        since: None,
        reverse: false,
    };
    let mut buf = Vec::new();
    hr.encode_length_delimited(&mut buf);
    println!("Hello, world! {:?}", buf);
}
