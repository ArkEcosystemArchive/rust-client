use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::api::models::delegate::Delegate;
use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub nonce: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub balance: u64,
    pub attributes: Attributes,
    pub is_delegate: bool,
    pub is_resigned: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_public_key: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_public_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipfs: Option<Hashes>,
    #[serde(skip)]
    pub delegate: Delegate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hashes {
    pub hashes: HashMap<String, bool>,
}

pub type Balances = HashMap<String, u64>;
