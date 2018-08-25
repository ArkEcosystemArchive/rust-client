extern crate failure;

use client::Client;

pub struct Accounts {
    client: Client
}

impl Accounts {

    pub fn new(client: Client) -> Accounts {
        Accounts { client }
    }

    pub fn delegates_fee(self) -> Result<String, failure::Error> {
        self.client.get("accounts/delegates/fee")
    }
}
