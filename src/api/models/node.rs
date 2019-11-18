use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use crate::api::models::fee::FeeSchema;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfiguration {
    pub core: HashMap<String, String>,
    #[serde(rename = "nethash")]
    pub nethash: String,
    pub slip44: u32,
    pub wif: u32,
    pub token: String,
    pub symbol: String,
    pub explorer: String,
    pub version: u32,
    // TODO: handle p2p null value return port? // custom serde method
    pub ports: HashMap<String, Option<u16>>,
    pub constants: NodeConstants,
    pub transaction_pool: TransactionPool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub synced: bool,
    pub now: u64,
    pub blocks_count: i64,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NodeSyncing {
    pub syncing: bool,
    pub blocks: i64,
    pub height: u64,
    pub id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConstants {
    pub height: u64,
    pub reward: u64,
    pub active_delegates: u32,
    pub blocktime: u32,
    pub block: NodeBlock,
    pub epoch: String,
    pub fees: Fees,
    pub ignore_invalid_second_signature_field: bool,
    pub ignore_expired_transactions: bool,
    pub vendor_field_length: u32,
    pub multi_payment_limit: u32,
    pub p2p: HashMap<String, Vec<String>>,
    pub aip11: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBlock {
    pub version: u32,
    pub max_transactions: u64,
    pub max_payload: u64,
    pub accept_expired_transaction_timestamps: bool,
    pub id_full_sha256: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub static_fees: FeeSchema,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFees {
    pub enabled: bool,
    pub min_fee_pool: u64,
    pub min_fee_broadcast: u64,
    pub addon_bytes: FeeSchema,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPool {
    pub dynamic_fees: DynamicFees,
}
