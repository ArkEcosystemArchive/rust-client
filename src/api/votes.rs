use http::client::Client;
use std::borrow::Borrow;

use api::models::Transaction;
use api::Result;

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub fn all(&self) -> Result<Vec<Transaction>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("votes", parameters)
    }

    pub fn show(&self, id: &str) -> Result<Transaction> {
        let endpoint = format!("votes/{}", id);
        self.client.get(&endpoint)
    }
}
