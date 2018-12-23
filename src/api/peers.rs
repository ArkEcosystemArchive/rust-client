use std::borrow::Borrow;

use api::ApiResult;
use api::models::{Peer, Response};
use http::client::Client;

pub struct Peers {
    client: Client,
}

impl Peers {
    pub fn new(client: Client) -> Peers {
        Peers { client }
    }

    pub fn all(&self) -> ApiResult<Response<Vec<Peer>>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> ApiResult<Response<Vec<Peer>>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("peers", parameters)
    }

    pub fn show(&self, ip_addr: &str) -> ApiResult<Response<Peer>> {
        let endpoint = format!("peers/{}", ip_addr);
        self.client.get(&endpoint)
    }
}
