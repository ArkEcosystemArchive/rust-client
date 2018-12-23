use failure;
use http::client::Client;
use std::borrow::Borrow;

use api::models::{Block, Response, Transaction};

pub struct Blocks {
    client: Client,
}

impl Blocks {
    pub fn new(client: Client) -> Blocks {
        Blocks { client }
    }

    pub fn all(&self) -> Result<Response<Vec<Block>>, failure::Error> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Block>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("blocks", parameters)
    }

    pub fn show(&self, id: &str) -> Result<Response<Block>, failure::Error> {
        let endpoint = format!("blocks/{}", id);

        self.client.get(&endpoint)
    }

    pub fn transactions(&self, id: &str) -> Result<Response<Vec<Transaction>>, failure::Error> {
        self.transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn transactions_params<I, K, V>(
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
        let endpoint = format!("blocks/{}/transactions", id);
        self.client
            .get_with_params(&endpoint, parameters)
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
    }
}
