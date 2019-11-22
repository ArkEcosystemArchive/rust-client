use serde::Deserialize;
use serde::Serialize;

use crate::api::models::asset::Asset;
use crate::api::models::timestamp::Timestamp;
use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u16>,
    #[serde(rename = "type")]
    pub transaction_type: u16,
    pub type_group: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub amount: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub fee: u64,
    pub sender: String,
    pub sender_public_key: String,
    pub recipient: Option<String>,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Asset>,
    pub confirmations: u64,
    pub timestamp: Timestamp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPostResponse {
    pub accept: Vec<String>,
    pub broadcast: Vec<String>,
    pub excess: Vec<String>,
    pub invalid: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPostError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionTypes {
    #[serde(rename = "1")]
    pub core: TransactionTypesCore,
    #[serde(rename = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magistrate: Option<TransactionTypesMagistrate>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionTypesCore {
    pub transfer: u16,
    pub second_signature: u16,
    pub delegate_registration: u16,
    pub vote: u16,
    pub multi_signature: u16,
    pub ipfs: u16,
    pub multi_payment: u16,
    pub delegate_resignation: u16,
    pub htlc_lock: u16,
    pub htlc_claim: u16,
    pub htlc_refund: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionTypesMagistrate {
    pub business_registration: u16,
    pub business_resignation: u16,
    pub business_update: u16,
    pub bridgechain_registration: u16,
    pub bridgechain_resignation: u16,
    pub bridgechain_update: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFees {
    #[serde(rename = "1")]
    pub core: TransactionFeesCore,
    #[serde(rename = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magistrate: Option<TransactionFeesMagistrate>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFeesCore {
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub transfer: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub second_signature: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub delegate_registration: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub vote: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub multi_signature: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub ipfs: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub multi_payment: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub delegate_resignation: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub htlc_lock: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub htlc_claim: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub htlc_refund: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFeesMagistrate {
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub business_registration: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub business_resignation: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub business_update: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub bridgechain_registration: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub bridgechain_resignation: u64,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub bridgechain_update: u64,
}
