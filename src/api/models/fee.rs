use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FeeStatistics {
    #[serde(rename = "type")]
    // pub transaction_type: TransactionType,
    pub fees: FeeStats,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeStats {
    pub min_fee: u64,
    pub max_fee: u64,
    pub avg_fee: u64,
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
