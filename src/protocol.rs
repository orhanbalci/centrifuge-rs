//include!(concat!(env!("OUT_DIR"), "/centrifugal.centrifuge.protocol.rs"));

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Error {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Command {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(enumeration = "command::MethodType", tag = "2")]
    pub method: i32,
    #[prost(bytes = "vec", tag = "3")]
    pub params: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Command`.
pub mod command {
    use serde::{Deserialize, Serialize};
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
        Serialize,
        Deserialize,
    )]
    #[repr(i32)]
    pub enum MethodType {
        Connect = 0,
        Subscribe = 1,
        Unsubscribe = 2,
        Publish = 3,
        Presence = 4,
        PresenceStats = 5,
        History = 6,
        Ping = 7,
        Send = 8,
        Rpc = 9,
        Refresh = 10,
        SubRefresh = 11,
    }
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Reply {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
    #[prost(bytes = "vec", tag = "3")]
    pub result: ::prost::alloc::vec::Vec<u8>,
}

#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Push {
    #[prost(enumeration = "push::PushType", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub channel: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `Push`.
pub mod push {
    use serde::{Deserialize, Serialize};
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
        Serialize,
        Deserialize,
    )]
    #[repr(i32)]
    pub enum PushType {
        Publication = 0,
        Join = 1,
        Leave = 2,
        Unsubscribe = 3,
        Message = 4,
        Subscribe = 5,
        Connect = 6,
        Disconnect = 7,
        Refresh = 8,
    }
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ClientInfo {
    #[prost(string, tag = "1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub client: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub conn_info: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub chan_info: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Publication {
    /// 1-3 skipped here for backwards compatibility.
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub info: ::core::option::Option<ClientInfo>,
    #[prost(uint64, tag = "6")]
    pub offset: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Join {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<ClientInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Leave {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<ClientInfo>,
}
/// Field 1 removed (bool resubscribe).
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Unsubscribe {}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Subscribe {
    #[prost(bool, tag = "1")]
    pub recoverable: bool,
    /// 2-3 skipped here for backwards compatibility.
    #[prost(string, tag = "4")]
    pub epoch: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub offset: u64,
    #[prost(bool, tag = "6")]
    pub positioned: bool,
    #[prost(bytes = "vec", tag = "7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Message {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Connect {
    #[prost(string, tag = "1")]
    pub client: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, message", tag = "4")]
    pub subs: ::std::collections::HashMap<::prost::alloc::string::String, SubscribeResult>,
    #[prost(bool, tag = "5")]
    pub expires: bool,
    #[prost(uint32, tag = "6")]
    pub ttl: u32,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Disconnect {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub reconnect: bool,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Refresh {
    #[prost(bool, tag = "1")]
    pub expires: bool,
    #[prost(uint32, tag = "2")]
    pub ttl: u32,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ConnectRequest {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, message", tag = "3")]
    pub subs: ::std::collections::HashMap<::prost::alloc::string::String, SubscribeRequest>,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ConnectResult {
    #[prost(string, tag = "1")]
    pub client: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub expires: bool,
    #[prost(uint32, tag = "4")]
    pub ttl: u32,
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(map = "string, message", tag = "6")]
    pub subs: ::std::collections::HashMap<::prost::alloc::string::String, SubscribeResult>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshRequest {
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct RefreshResult {
    #[prost(string, tag = "1")]
    pub client: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub expires: bool,
    #[prost(uint32, tag = "4")]
    pub ttl: u32,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct SubscribeRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub recover: bool,
    /// 4-5 skipped here for backwards compatibility.
    #[prost(string, tag = "6")]
    pub epoch: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub offset: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct SubscribeResult {
    #[prost(bool, tag = "1")]
    pub expires: bool,
    #[prost(uint32, tag = "2")]
    pub ttl: u32,
    #[prost(bool, tag = "3")]
    pub recoverable: bool,
    /// 4-5 skipped here for backwards compatibility.
    #[prost(string, tag = "6")]
    pub epoch: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub publications: ::prost::alloc::vec::Vec<Publication>,
    #[prost(bool, tag = "8")]
    pub recovered: bool,
    #[prost(uint64, tag = "9")]
    pub offset: u64,
    #[prost(bool, tag = "10")]
    pub positioned: bool,
    #[prost(bytes = "vec", tag = "11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct SubRefreshRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct SubRefreshResult {
    #[prost(bool, tag = "1")]
    pub expires: bool,
    #[prost(uint32, tag = "2")]
    pub ttl: u32,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct UnsubscribeRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct UnsubscribeResult {}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PublishRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PublishResult {}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PresenceRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PresenceResult {
    #[prost(map = "string, message", tag = "1")]
    pub presence: ::std::collections::HashMap<::prost::alloc::string::String, ClientInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PresenceStatsRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PresenceStatsResult {
    #[prost(uint32, tag = "1")]
    pub num_clients: u32,
    #[prost(uint32, tag = "2")]
    pub num_users: u32,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct StreamPosition {
    #[prost(uint64, tag = "1")]
    pub offset: u64,
    #[prost(string, tag = "2")]
    pub epoch: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct HistoryRequest {
    #[prost(string, tag = "1")]
    pub channel: ::prost::alloc::string::String,
    /// 2-6 skipped here for backwards compatibility.
    #[prost(int32, tag = "7")]
    pub limit: i32,
    #[prost(message, optional, tag = "8")]
    pub since: ::core::option::Option<StreamPosition>,
    #[prost(bool, tag = "9")]
    pub reverse: bool,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct HistoryResult {
    #[prost(message, repeated, tag = "1")]
    pub publications: ::prost::alloc::vec::Vec<Publication>,
    #[prost(string, tag = "2")]
    pub epoch: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub offset: u64,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PingRequest {}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct PingResult {}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct RpcRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub method: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct RpcResult {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct SendRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
