use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bridgechain {
    pub public_key: String,
    pub name: String,
    pub seed_nodes: Vec<String>,
    pub genesis_hash: String,
    pub bridgechain_repository: String,
    pub ports: HashMap<String, u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
}
