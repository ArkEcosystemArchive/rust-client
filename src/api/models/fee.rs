use serde::Deserialize;
use serde::Serialize;

use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeStats {
    #[serde(rename = "type")]
    pub transaction_type: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub avg: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub min: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub max: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub sum: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeSchema {
    pub transfer: u64,
    pub second_signature: u64,
    pub delegate_registration: u64,
    pub vote: u64,
    pub multi_signature: u64,
    pub ipfs: u64,
    pub multi_payment: u64,
    pub delegate_resignation: u64,
    pub htlc_lock: u64,
    pub htlc_claim: u64,
    pub htlc_refund: u64,
}
