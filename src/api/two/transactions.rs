use failure;
use http::client::Client;
use serde_json::from_value;
use std::borrow::Borrow;

use api::two::models::{Response, Transaction, TransactionTypes};

pub struct Transactions {
    client: Client,
}

impl Transactions {
    pub fn new(client: Client) -> Transactions {
        Transactions { client }
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("transactions", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn create<I, K, V>(&self, payload: I) -> Result<Response<Transaction>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post("transactions", Some(payload))
            .map(|v| from_value(v).unwrap())
    }

    pub fn show(&self, id: String) -> Result<Response<Transaction>, failure::Error> {
        let endpoint = format!("transactions/{}", id);
        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }

    pub fn all_unconfirmed<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("transactions/unconfirmed", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn show_unconfirmed(
        &self,
        id: String,
    ) -> Result<Response<Vec<Transaction>>, failure::Error> {
        let endpoint = format!("transactions/unconfirmed/{}", id);
        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }

    pub fn search<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("transactions/search", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn types(&self) -> Result<Response<TransactionTypes>, failure::Error> {
        self.client
            .get("transactions/types")
            .map(|v| from_value(v).unwrap())
    }
}
