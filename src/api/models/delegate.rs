use serde::Deserialize;
use serde::Serialize;

use crate::api::models::timestamp::Timestamp;
use crate::common::deserialize_u64_as_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub username: String,
    pub address: String,
    pub public_key: String,
    pub votes: String,
    pub rank: u32,
    pub blocks: Blocks,
    pub production: Production,
    pub forged: DelegateForged,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Blocks {
    pub produced: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<Last>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Production {
    pub approval: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DelegateForged {
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub rewards: u64,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub fees: u64,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub total: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Last {
    pub id: String,
    pub height: u64,
    pub timestamp: Timestamp,
}
