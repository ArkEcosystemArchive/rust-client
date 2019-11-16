use crate::http::client::Client;
use std::borrow::Borrow;

use crate::api::models::block::Block;
use crate::api::models::transaction::Transaction;

use crate::api::Result;

pub struct Blocks {
    client: Client,
}

impl Blocks {
    pub fn new(client: Client) -> Blocks {
        Blocks { client }
    }

    pub fn all(&mut self) -> Result<Vec<Block>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Block>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("blocks", parameters)
    }

    pub fn show(&mut self, id: &str) -> Result<Block> {
        let endpoint = format!("blocks/{}", id);

        self.client.get(&endpoint)
    }

    pub fn transactions(&mut self, id: &str) -> Result<Vec<Transaction>> {
        self.transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn transactions_params<I, K, V>(
        &mut self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("blocks/{}/transactions", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn search<I, K, V>(&mut self, parameters: I) -> Result<Vec<Block>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("blocks/search", parameters)
    }
}
