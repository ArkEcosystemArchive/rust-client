use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::api::models::business::Business;
use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub public_key: Option<String>,
    pub username: Option<String>,
    pub second_public_key: Option<String>,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub balance: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub nonce: u64,
    pub is_delegate: bool,
    pub is_resigned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<Business>,
}

pub type Balances = HashMap<String, u64>;
