use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::common::deserialize_u64_as_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub public_key: Option<String>,
    pub username: Option<String>,
    pub second_public_key: Option<String>,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub balance: u64,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub nonce: u64,
    pub is_delegate: bool,
    pub is_resigned: bool,
}

pub type Balances = HashMap<String, u64>;
