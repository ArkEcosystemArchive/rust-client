use serde::Deserialize;
use serde::Serialize;

use crate::common::deserialize_as_u64_from_number_or_string;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkFeeStats {
    #[serde(rename = "1")]
    pub core: CoreFeeStats,
    #[serde(rename = "2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magistrate: Option<MagistrateFeeStats>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeStatistics {
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
pub struct CoreFeeStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signature: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_registration: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_signature: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipfs: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_payment: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegate_resignation: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htlc_lock: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htlc_claim: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub htlc_refund: Option<FeeStatistics>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MagistrateFeeStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_resignation: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_update: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridgechain_registration: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridgechain_resignation: Option<FeeStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridgechain_update: Option<FeeStatistics>,
}
