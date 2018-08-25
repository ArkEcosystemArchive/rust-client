extern crate failure;

use client::Client;
use std::borrow::Borrow;

pub struct Wallets {
    client: Client
}

impl Wallets {

    pub fn new(client: Client) -> Wallets {
        Wallets { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("wallets", parameters)
    }

    pub fn top<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("wallets/top", parameters)
    }

    pub fn show(self, id: String) -> Result<String, failure::Error> {
        let endpoint = format!("wallets/{}", id);
        self.client.get(&endpoint)
    }

    pub fn transactions<I, K, V>(self, id: String, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let endpoint = format!("wallets/{}/transactions", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn sent_transactions<I, K, V>(self, id: String, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let endpoint = format!("wallets/{}/transactions/sent", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn received_transactions<I, K, V>(self, id: String, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let endpoint = format!("wallets/{}/transactions/received", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn votes(self, id: String) -> Result<String, failure::Error> {
        let endpoint = format!("wallets/{}/votes", id);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("wallets/search", parameters)
    }
}
