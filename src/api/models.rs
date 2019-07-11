use serde::de::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestError {
    pub status_code: i16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    pub data: T,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub count: u32,
    pub page_count: u32,
    pub total_count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    #[serde(rename = "self")]
    pub self_url: String,
    pub first: String,
    pub last: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: String,
    pub version: u8,
    pub height: u64,
    pub previous: String,
    pub forged: Forged,
    pub payload: Payload,
    pub generator: Generator,
    pub signature: String,
    pub transactions: u32,
    pub confirmations: u64,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Forged {
    pub reward: String,
    pub fee: String,
    pub total: String,
    pub amount: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Payload {
    pub hash: String,
    pub length: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Generator {
    pub username: String,
    pub address: String,
    pub public_key: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Timestamp {
    pub epoch: u32,
    pub unix: u32,
    pub human: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Delegate {
    pub username: String,
    pub address: String,
    pub public_key: String,
    pub votes: u64,
    pub rank: u32,
    pub blocks: Blocks,
    pub production: Production,
    pub forged: DelegateForged,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Blocks {
    pub produced: u64,
    pub missed: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<Last>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Production {
    pub approval: f64,
    pub productivity: f64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Last {
    pub id: String,
    pub timestamp: Timestamp,
}

pub type Balances = HashMap<String, u64>;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct DelegateForged {
    pub rewards: u64,
    pub fees: u64,
    pub total: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConfiguration {
    #[serde(rename = "nethash")]
    pub nethash: String,
    pub token: String,
    pub symbol: String,
    pub explorer: String,
    pub version: u32,
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
    pub timestamp: u32,
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
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeBlock {
    pub version: u32,
    pub max_transactions: u64,
    pub max_payload: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionPool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_fees: Option<DynamicFees>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicFees {
    pub min_fee_pool: u64,
    pub min_fee_broadcast: u64,
    pub addon_bytes: FeeSchema,
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
    pub timelock_transfer: u64,
    pub multi_payment: u64,
    pub delegate_resignation: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct FeeStatistics {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub fees: FeeStats,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeStats {
    pub min_fee: String,
    pub max_fee: String,
    pub avg_fee: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
    pub ports: HashMap<String, i16>,
    pub version: String,
    pub height: u64,
    pub latency: u32,
}

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
    pub amount: String,
    pub fee: String,
    pub sender_public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_public_key: Option<String>,
    pub sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    pub signature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_field: Option<String>,
    #[serde(skip_serializing_if = "Asset::is_none")]
    pub asset: Asset,
    pub timestamp: Timestamp,
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub public_key: Option<String>,
    pub username: Option<String>,
    pub second_public_key: Option<String>,
    pub balance: String,
    pub is_delegate: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Asset {
    #[serde(skip)]
    None,
    Signature {
        #[serde(rename = "publicKey")]
        public_key: String,
    },
    Delegate {
        username: String,
    },
    Votes(Vec<String>),
    #[serde(rename = "multisignature")]
    MultiSignatureRegistration {
        min: u8,
        keysgroup: Vec<String>,
        lifetime: u8,
    },
}

impl Asset {
    pub fn is_none(&self) -> bool {
        match *self {
            Asset::None => true,
            _ => false,
        }
    }
}

impl Default for Asset {
    fn default() -> Self {
        Asset::None
    }
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
