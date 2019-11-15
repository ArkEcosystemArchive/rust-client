use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use std::collections::HashMap;

use crate::api::models_new::timestamp::Timestamp;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestError {
    pub status_code: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub data: T,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub total_count_is_estimate: bool,
    pub count: u32,
    pub page_count: u32,
    pub total_count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    #[serde(rename = "self")]
    pub self_url: String,
    pub first: String,
    pub last: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub username: String,
    pub address: String,
    pub public_key: String,
    pub votes: u64,
    pub rank: u32,
    pub blocks: Blocks,
    pub production: Production,
    pub forged: DelegateForged,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Blocks {
    pub produced: u64,
    pub missed: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<Last>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Production {
    pub approval: f64,
    pub productivity: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Last {
    pub id: String,
    pub timestamp: Timestamp,
}

pub type Balances = HashMap<String, u64>;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DelegateForged {
    pub rewards: u64,
    pub fees: u64,
    pub total: u64,
}

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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub public_key: Option<String>,
    pub username: Option<String>,
    pub second_public_key: Option<String>,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub balance: u64,
    pub is_delegate: bool,
}

fn deserialize_u64_as_number_or_string<'de, D>(de: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let deser_result: serde_json::Value = Deserialize::deserialize(de)?;

    match deser_result {
        serde_json::Value::Number(ref obj) if obj.is_u64() => Ok(obj.as_u64().unwrap()),
        serde_json::Value::String(ref obj) if !obj.is_empty() => {
            Ok(obj.as_str().parse::<u64>().unwrap())
        }
        _ => Ok(0),
    }
}
