use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    pub ip: String,
    pub port: i16,
    pub ports: HashMap<String, i16>,
    pub version: String,
    pub height: u64,
    pub latency: u32,
}
