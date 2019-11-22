use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Asset {
    None,
    Signature {
        #[serde(rename = "publicKey")]
        public_key: String,
    },
    Delegate {
        username: String,
    },
    Votes(Vec<String>),
    #[serde(rename = "multiSignature")]
    MultiSignatureRegistration {
        #[serde(rename = "publicKeys")]
        public_keys: Vec<String>,
        min: u8,
    },
    Ipfs(String),
    Payments(Vec<Payment>),
    Lock {
        #[serde(rename = "secretHash")]
        secret_hash: String,
        expiration: Expiration,
    },
    Claim {
        #[serde(rename = "lockTransactionId")]
        lock_transaction_id: String,
        #[serde(rename = "unlockSecret")]
        unlock_secret: String,
    },
    #[serde(rename = "refund")]
    Refund {
        #[serde(rename = "lockTransactionId")]
        lock_transaction_id: String,
    },
    #[serde(rename = "businessRegistration")]
    BusinessRegistration {
        name: String,
        website: String,
    },
    #[serde(rename = "businessUpdate")]
    BusinessUpdate {
        name: String,
        website: String,
    },
    #[serde(rename = "bridgechainRegistration")]
    BridgeChainRegistration {
        name: String,
        #[serde(rename = "seedNodes")]
        seed_nodes: Vec<String>,
        #[serde(rename = "genesisHash")]
        genesis_hash: String,
        #[serde(rename = "bridgechainRepository")]
        bridgechain_repository: String,
        ports: HashMap<String, u32>,
    },
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expiration {
    #[serde(rename = "type")]
    pub expiration_type: u64,
    pub value: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub amount: String,
    pub recipient_id: String,
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
