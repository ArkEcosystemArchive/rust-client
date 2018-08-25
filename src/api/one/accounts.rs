extern crate failure;

use client::Client;

pub struct Accounts {
    client: Client
}

impl Accounts {

    pub fn new(client: Client) -> Accounts {
        Accounts { client }
    }

    pub fn balance(self, address: String) -> Result<String, failure::Error> {
        self.client.get_with_params("accounts/getBalance", &[("address", &address)])
    }

    pub fn public_key(self, address: String) -> Result<String, failure::Error> {
        self.client.get_with_params("accounts/getPublicKey", &[("address", &address)])
    }

    pub fn delegate(self, address: String) -> Result<String, failure::Error> {
        self.client.get_with_params("accounts/delegates", &[("address", &address)])
    }

    pub fn delegates_fee(self) -> Result<String, failure::Error> {
        self.client.get("accounts/delegates/fee")
    }

    pub fn account(self, address: String) -> Result<String, failure::Error> {
        self.client.get_with_params("accounts", &[("address", &address)])
    }
}
