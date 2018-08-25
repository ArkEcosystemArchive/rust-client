use failure;
use client::Client;
use serde_json::Value;

pub struct Signatures {
    client: Client
}

impl Signatures {

    pub fn new(client: Client) -> Signatures {
        Signatures { client }
    }

    pub fn fee(self) -> Result<Value, failure::Error>
    {
        self.client.get("signatures/fee")
    }
}
