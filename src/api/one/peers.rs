use failure;
use std::borrow::Borrow;
use http::client::Client;
use serde_json::Value;

pub struct Peers {
    client: Client
}

impl Peers {

    pub fn new(client: Client) -> Peers {
        Peers { client }
    }

    pub fn status(&self, ip_addr: String, port: String) -> Result<Value, failure::Error>
    {
        let params = &[("ip".to_owned(), ip_addr), ("port".to_owned(), port)];
        self.client.get_with_params("peers/get", params)
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
             I::Item: Borrow<(K, V)>,
             K: AsRef<str>,
             V: AsRef<str>
    {
        self.client.get_with_params("peers", parameters)
    }

    pub fn version(&self) -> Result<Value, failure::Error>
    {
        self.client.get("peers/version")
    }
}
