use serde::Deserialize;
use serde::Serialize;

use crate::api::models::timestamp::Timestamp;
use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    pub lock_id: String,
    #[serde(deserialize_with = "deserialize_as_u64_from_number_or_string")]
    pub amount: u64,
    pub secret_hash: String,
    pub sender_public_key: String,
    pub recipient_id: String,
    pub timestamp: Timestamp,
    pub expiration_type: u8,
    pub expiration_value: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_field: Option<String>,
}
