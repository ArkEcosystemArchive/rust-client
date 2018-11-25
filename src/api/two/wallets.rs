use failure;
use http::client::Client;
use serde_json::from_value;
use std::borrow::Borrow;

use api::two::models::{Response, Transaction, Wallet};

pub struct Wallets {
    client: Client,
}

impl Wallets {
    pub fn new(client: Client) -> Wallets {
        Wallets { client }
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Wallet>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("wallets", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn top<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Wallet>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("wallets/top", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn show(&self, id: &str) -> Result<Response<Wallet>, failure::Error> {
        let endpoint = format!("wallets/{}", id);
        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }

    pub fn transactions<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions", id);
        self.client
            .get_with_params(&endpoint, parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn sent_transactions<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions/sent", id);
        self.client
            .get_with_params(&endpoint, parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn received_transactions<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions/received", id);
        self.client
            .get_with_params(&endpoint, parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn votes(&self, id: &str) -> Result<Response<Vec<Transaction>>, failure::Error> {
        let endpoint = format!("wallets/{}/votes", id);
        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }

    pub fn search<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Wallet>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("wallets/search", parameters)
            .map(|v| from_value(v).unwrap())
    }
}
