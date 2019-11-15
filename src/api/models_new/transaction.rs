use serde::Deserialize;
use serde::Serialize;

use crate::api::models_new::asset::Asset;
use crate::api::models_new::timestamp::Timestamp;

use crate::common::deserialize_u64_as_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub block_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u16>,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_group: Option<u16>,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub amount: u64,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub fee: u64,
    pub sender: String,
    pub sender_public_key: String,
    pub recipient: Option<String>,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_field: Option<String>,
    #[serde(skip_serializing_if = "Asset::is_none")]
    pub asset: Asset,
    pub confirmations: u64,
    pub timestamp: Timestamp,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub nonce: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFees {
    pub transfer: u64,
    pub second_signature: u64,
    pub delegate_registration: u64,
    pub vote: u64,
    pub multi_signature: u64,
    pub ipfs: u64,
    pub timelock_transfer: u64,
    pub multi_payment: u64,
    pub delegate_resignation: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionTypes {
    pub transfer: u16,
    pub second_signature: u16,
    pub delegate_registration: u16,
    pub vote: u16,
    pub multi_signature: u16,
    pub ipfs: u16,
    pub timelock_transfer: u16,
    pub multi_payment: u16,
    pub delegate_resignation: u16,
}

enum_number!(TransactionType {
    Transfer = 0,
    SecondSignatureRegistration = 1,
    DelegateRegistration = 2,
    Vote = 3,
    MultiSignatureRegistration = 4,
    Ipfs = 5,
    TimelockTransfer = 6,
    MultiPayment = 7,
    DelegateResignation = 8,
    HtlcLock = 9,
    HtlcClaim = 10,
    HtlcRefund = 11,
});

use std::mem::transmute;
impl From<u8> for TransactionType {
    fn from(t: u8) -> TransactionType {
        assert!(
            TransactionType::Transfer as u8 <= t && t <= TransactionType::DelegateResignation as u8
        );
        unsafe { transmute(t) }
    }
}

impl Default for TransactionType {
    fn default() -> Self {
        TransactionType::Transfer
    }
}
