use failure;
use http::client::Client;
use std::borrow::Borrow;
use serde_json::Value;

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub fn all<I, K, V>(&self, parameters: I) -> Result<Value, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("votes", parameters)
    }

    pub fn show(&self, id: String) -> Result<Value, failure::Error> {
        let endpoint = format!("votes/{}", id);
        self.client.get(&endpoint)
    }
}
