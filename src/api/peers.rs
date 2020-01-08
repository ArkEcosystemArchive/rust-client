use std::borrow::Borrow;

use crate::api::models::peer::Peer;
use crate::api::Result;
use crate::http::client::Client;

pub struct Peers {
    client: Client,
}

impl Peers {
    pub fn new(client: Client) -> Peers {
        Peers { client }
    }

    pub async fn all(&mut self) -> Result<Vec<Peer>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Peer>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("peers", parameters).await
    }

    pub async fn show(&mut self, ip_address: &str) -> Result<Peer> {
        let endpoint = format!("peers/{}", ip_address);
        self.client.get(&endpoint).await
    }
}
