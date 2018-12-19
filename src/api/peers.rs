use failure;
use serde_json::from_value;
use std::borrow::Borrow;

use http::client::Client;
use api::models::{Peer, Response};

pub struct Peers {
    client: Client,
}

impl Peers {
    pub fn new(client: Client) -> Peers {
        Peers { client }
    }

    pub fn all(&self) -> Result<Response<Vec<Peer>>, failure::Error>
    {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Peer>>, failure::Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("peers", parameters)
            .map(|v| from_value(v).unwrap())
    }

    pub fn show(&self, ip_addr: &str) -> Result<Response<Peer>, failure::Error> {
        let endpoint = format!("peers/{}", ip_addr);
        self.client
            .get(&endpoint)
            .map(|v| from_value(v).unwrap())
    }
}
