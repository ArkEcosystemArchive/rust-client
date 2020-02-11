use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub address: String,
    pub public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat: Option<String>,
    pub name: String,
    pub website: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
}
