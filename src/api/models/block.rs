use serde::Deserialize;
use serde::Serialize;

use crate::api::models::timestamp::Timestamp;
use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub version: u8,
    pub height: u64,
    pub previous: String,
    pub forged: Forged,
    pub payload: Payload,
    pub generator: Generator,
    pub signature: String,
    pub confirmations: u32,
    pub transactions: u32,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Forged {
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub reward: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub fee: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub total: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub amount: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Payload {
    pub hash: String,
    pub length: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: String,
    pub address: String,
    pub public_key: String,
}
