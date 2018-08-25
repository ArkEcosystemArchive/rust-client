use failure;
use std::borrow::Borrow;
use client::Client;
use serde_json::Value;

pub struct Transactions {
    client: Client
}

impl Transactions {

    pub fn new(client: Client) -> Transactions {
        Transactions { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
             I::Item: Borrow<(K, V)>,
             K: AsRef<str>,
             V: AsRef<str>
    {
        self.client.get_with_params("transactions", parameters)
    }

    pub fn create<I, K, V>(self, payload: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
             I::Item: Borrow<(K, V)>,
             K: AsRef<str>,
             V: AsRef<str>
    {
        self.client.post("transactions", Some(payload))
    }

    pub fn show(self, id: String) -> Result<Value, failure::Error>
    {
        let endpoint = format!("transactions/{}", id);
        self.client.get(&endpoint)
    }

    pub fn all_unconfirmed<I, K, V>(self, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
             I::Item: Borrow<(K, V)>,
             K: AsRef<str>,
             V: AsRef<str>
    {
        self.client.get_with_params("transactions/unconfirmed", parameters)
    }

    pub fn show_unconfirmed(self, id: String) -> Result<Value, failure::Error>
    {
        let endpoint = format!("transactions/unconfirmed/{}", id);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(self, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
             I::Item: Borrow<(K, V)>,
             K: AsRef<str>,
             V: AsRef<str>
    {
        self.client.get_with_params("transactions/search", parameters)
    }

    pub fn types(self) -> Result<Value, failure::Error>
    {
        self.client.get("transactions/types")
    }
}
