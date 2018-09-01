use failure;
use std::borrow::Borrow;
use http::client::Client;
use utils;
use serde_json::Value;

pub struct Delegates {
    client: Client,
}

impl Delegates {
    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("delegates", parameters)
    }

    pub fn show<I, K, V>(&self, parameters: I) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("delegates/get", parameters)
    }

    pub fn count(&self) -> Result<Value, failure::Error> {
        self.client.get("delegates/count")
    }

    pub fn search<I, K, V>(&self, query: String, parameters: I) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mandatory = &[("q".to_owned(), query)];
        let merged = utils::merge_params(mandatory, parameters);
        self.client.get_with_params("delegates/search", merged)
    }

    pub fn voters<I, K, V>(
        &self,
        public_key: String,
        parameters: I,
    ) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mandatory = &[("publicKey".to_owned(), public_key)];
        let merged = utils::merge_params(mandatory, parameters);
        self.client.get_with_params("delegates/voters", merged)
    }

    pub fn fee(&self) -> Result<Value, failure::Error> {
        self.client.get("delegates/fee")
    }

    pub fn forged_by_account(&self, generator_public_key: String) -> Result<Value, failure::Error> {
        self.client.get_with_params(
            "delegates/forging/getForgedByAccount",
            &[("generatorPublicKey", &generator_public_key)],
        )
    }

    pub fn next_forgers(&self) -> Result<Value, failure::Error> {
        self.client.get("delegates/getNextForgers")
    }

    pub fn forging_status<I, K, V>(
        &self,
        public_key: String,
        parameters: I,
    ) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let mandatory = &[("publicKey".to_owned(), public_key)];
        let merged = utils::merge_params(mandatory, parameters);
        self.client
            .get_with_params("delegates/forging/status", merged)
    }
}
