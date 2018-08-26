use failure;
use std::borrow::Borrow;
use http::client::Client;
use serde_json::Value;

pub struct Transactions {
    client: Client
}

impl Transactions {

    pub fn new(client: Client) -> Transactions {
        Transactions { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("transactions", parameters)
    }

    pub fn show(self, id: String) -> Result<Value, failure::Error>
    {
        let params = &[("id".to_owned(), id)];
        self.client.get_with_params("transactions/get", params)
    }

    pub fn all_unconfirmed<I, K, V>(self, parameters: I) -> Result<Value, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
            self.client.get_with_params("transactions/unconfirmed", parameters)
    }

    pub fn show_unconfirmed(self, id: String) -> Result<Value, failure::Error>
    {
        let params = &[("id".to_owned(), id)];
        self.client.get_with_params("transactions/unconfirmed/get", params)
    }
}
