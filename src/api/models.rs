use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
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
    pub timestamp: Timestamp,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Forged {
    pub reward: u64,
    pub fee: u64,
    pub total: u64,
    pub amount: u64,
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
pub struct Peer {
    pub ip: String,
    pub port: u16,
    pub version: String,
    pub height: u64,
    pub status: u16,
    pub os: String,
    pub latency: u32,
    pub hashid: String,
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
    pub amount: u64,
    pub fee: u64,
    pub sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(deserialize_with = "deserialize_u64_as_number_or_string")]
    pub balance: u64,
    pub is_delegate: bool,
}

fn deserialize_u64_as_number_or_string<'de, D>(de: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let deser_result: serde_json::Value = Deserialize::deserialize(de)?;

    match deser_result {
        serde_json::Value::Number(ref obj) if obj.is_u64() => Ok(obj.as_u64().unwrap()),
        serde_json::Value::String(ref obj) if !obj.is_empty() => {
            Ok(obj.as_str().parse::<u64>().unwrap())
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
