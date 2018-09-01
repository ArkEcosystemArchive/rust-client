use failure;
use http::client::Client;
use std::borrow::Borrow;
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

    pub fn show(&self, id: String) -> Result<Value, failure::Error> {
        let endpoint = format!("delegates/{}", id);
        self.client.get(&endpoint)
    }

    pub fn blocks<I, K, V>(&self, id: String, parameters: I) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/blocks", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn voters<I, K, V>(&self, id: String, parameters: I) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/voters", id);
        self.client.get_with_params(&endpoint, parameters)
    }
}
