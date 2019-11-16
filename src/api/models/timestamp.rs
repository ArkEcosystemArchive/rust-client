use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Timestamp {
    pub epoch: u32,
    pub unix: u32,
    pub human: String,
}
