use failure;
use client::Client;
use std::borrow::Borrow;

pub struct Blocks {
    client: Client
}

impl Blocks {

    pub fn new(client: Client) -> Blocks {
        Blocks { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("blocks", parameters)
    }

    pub fn show(self, id: String) -> Result<String, failure::Error> {
        let endpoint = format!("blocks/{}", id);
        self.client.get(&endpoint)
    }

    pub fn transactions<I, K, V>(self, id: String, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let endpoint = format!("blocks/{}/transactions", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn search<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("blocks/search", parameters)
    }
}
