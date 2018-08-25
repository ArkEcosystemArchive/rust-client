extern crate failure;

use client::Client;
use std::borrow::Borrow;

pub struct Delegates {
    client: Client
}

impl Delegates {

    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("delegates", parameters)
    }

    pub fn show(self, id: String) -> Result<String, failure::Error> {
        let endpoint = format!("delegates/{}", id);
        self.client.get(&endpoint)
    }

    pub fn blocks<I, K, V>(self, id: String, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let endpoint = format!("delegates/{}/blocks", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn voters<I, K, V>(self, id: String, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        let endpoint = format!("delegates/{}/voters", id);
        self.client.get_with_params(&endpoint, parameters)
    }
}
