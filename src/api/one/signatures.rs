use failure;
use client::Client;

pub struct Signatures {
    client: Client
}

impl Signatures {

    pub fn new(client: Client) -> Signatures {
        Signatures { client }
    }

    pub fn fee(self) -> Result<String, failure::Error>
    {
        self.client.get("signatures/fee")
    }
}
