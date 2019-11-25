use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub public_key: String,
    pub name: String,
    pub website: String,
}
