use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
    pub version: String,
    pub height: u64,
    pub status: u16,
    pub os: String,
    pub latency: u32,
    pub hashid: String,
}
