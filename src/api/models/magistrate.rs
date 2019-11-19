use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub business_id: u64,
    pub name: String,
    pub website: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bridgechain {
    pub bridgechain_id: u64,
    pub business_id: u64,
    pub name: String,
    pub seed_nodes: Vec<String>,
    pub genesis_hash: String,
    pub bridgechain_repository: String,
}
