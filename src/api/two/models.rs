use serde::de::{Deserialize, Deserializer};
use serde_json;
use std::collections::HashMap;

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
    pub _self_: String,
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
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Forged {
    pub reward: u64,
    pub fee: u64,
    pub total: u64,
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
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Blocks {
    pub produced: u64,
    pub missed: u64,
    pub last: Last,
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

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
    pub dynamic: bool,
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
#[serde(rename_all = "camelCase")]
pub struct NodeConfiguration {
    #[serde(rename = "nethash")]
    pub nethash: String,
    pub token: String,
    pub symbol: String,
    pub explorer: String,
    pub version: u32,
    pub ports: HashMap<String, u16>,
    pub constants: NodeConstants,
    pub fee_statistics: Vec<FeeStatistics>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub synced: bool,
    pub now: u64,
    pub blocks_count: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct NodeSyncing {
    pub syncing: bool,
    pub blocks: u64,
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
    pub dynamic_offsets: DynamicOffsets,
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
pub struct DynamicOffsets {
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
    pub min_fee: u64,
    pub max_fee: u64,
    pub avg_fee: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Peer {
    pub ip: String,
    pub port: u16,
    pub version: String,
    pub height: u64,
    pub status: String,
    pub os: String,
    pub latency: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub block_id: String,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub amount: u64,
    pub fee: u64,
    pub sender: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub recipient: String,
    pub signature: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub vendor_field: String,
    #[serde(skip_serializing_if = "Asset::is_none")]
    pub asset: Asset,
    pub confirmations: u64,
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TransactionTypes {
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

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    pub address: String,
    pub public_key: Option<String>,
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub balance: i64,
    pub is_delegate: bool,
}

fn deserialize_u64_as_number_or_string<'de, D>(de: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let deser_res: serde_json::Value = try!(Deserialize::deserialize(de));

    match deser_res {
        serde_json::Value::Number(ref obj) if obj.is_i64() => Ok(obj.as_i64().unwrap()),
        serde_json::Value::String(ref obj) if obj.len() > 0 => {
            Ok(obj.as_str().parse::<i64>().unwrap())
        }
        _ => Ok(0),
    }
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
