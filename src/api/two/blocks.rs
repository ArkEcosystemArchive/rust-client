use failure;
use http::client::Client;
use serde_json::from_value;
use std::borrow::Borrow;

use api::two::models::{Block, Response, Transaction};

pub struct Blocks {
    client: Client,
}

impl Blocks {
    pub fn new(client: Client) -> Blocks {
        Blocks { client }
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Block>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("blocks", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn show(&self, id: String) -> Result<Response<Block>, failure::Error> {
        let endpoint = format!("blocks/{}", id);

        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }

    pub fn transactions<I, K, V>(
        &self,
        id: String,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("blocks/{}/transactions", id);
        self.client
            .get_with_params(&endpoint, parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn search<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Block>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("blocks/search", parameters)
            .map(|v| from_value(v).unwrap())
    }
}
