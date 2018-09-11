use failure;
use http::client::Client;
use serde_json::from_value;
use std::borrow::Borrow;

use api::two::models::{Block, Delegate, Response, Wallet};

pub struct Delegates {
    client: Client,
}

impl Delegates {
    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Delegate>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("delegates", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn show(&self, id: String) -> Result<Response<Delegate>, failure::Error> {
        let endpoint = format!("delegates/{}", id);
        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }

    pub fn blocks<I, K, V>(
        &self,
        id: String,
        parameters: I,
    ) -> Result<Response<Vec<Block>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/blocks", id);
        self.client
            .get_with_params(&endpoint, parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn voters<I, K, V>(
        &self,
        id: String,
        parameters: I,
    ) -> Result<Response<Vec<Wallet>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/voters", id);
        self.client
            .get_with_params(&endpoint, parameters)
            .map(|v| from_value(v).unwrap())
    }
}
