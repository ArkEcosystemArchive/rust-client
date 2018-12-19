use failure;
use http::client::Client;
use serde_json::from_value;
use std::borrow::Borrow;

use api::models::{Response, Transaction};

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub fn all(&self) -> Result<Response<Vec<Transaction>>, failure::Error> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("votes", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn show(&self, id: &str) -> Result<Response<Transaction>, failure::Error> {
        let endpoint = format!("votes/{}", id);
        self.client.get(&endpoint).map(|v| from_value(v).unwrap())
    }
}
