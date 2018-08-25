extern crate failure;

use std::borrow::Borrow;
use client::Client;

pub struct Peers {
    client: Client
}

impl Peers {

    pub fn new(client: Client) -> Peers {
        Peers { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
             I::Item: Borrow<(K, V)>,
             K: AsRef<str>,
             V: AsRef<str>
    {
        self.client.get_with_params("peers", parameters)
    }

    pub fn show(self, ip_addr: String) -> Result<String, failure::Error>
    {
        let endpoint = format!("delegates/{}", ip_addr);
        self.client.get(&endpoint)
    }
}
