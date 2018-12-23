use http::client::Client;
use std::borrow::Borrow;

use api::ApiResult;
use api::models::{Response, Transaction};

pub struct Votes {
    client: Client,
}

impl Votes {
    pub fn new(client: Client) -> Votes {
        Votes { client }
    }

    pub fn all(&self) -> ApiResult<Response<Vec<Transaction>>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(
        &self,
        parameters: I,
    ) -> ApiResult<Response<Vec<Transaction>>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("votes", parameters)
    }

    pub fn show(&self, id: &str) -> ApiResult<Response<Transaction>> {
        let endpoint = format!("votes/{}", id);
        self.client.get(&endpoint)
    }
}
