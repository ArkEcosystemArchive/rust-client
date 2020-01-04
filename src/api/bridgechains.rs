use crate::api::models::bridgechain::Bridgechain;
use crate::api::Result;
use crate::http::client::Client;
use std::borrow::Borrow;
use std::collections::HashMap;

pub struct Bridgechains {
    client: Client,
}

impl Bridgechains {
    pub fn new(client: Client) -> Bridgechains {
        Bridgechains { client }
    }

    pub async fn all(&mut self) -> Result<Vec<Bridgechain>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Bridgechain>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("bridgechains", parameters)
            .await
    }

    pub async fn show(&mut self, ip_addr: &str) -> Result<Bridgechain> {
        let endpoint = format!("bridgechains/{}", ip_addr);
        self.client.get(&endpoint).await
    }

    pub async fn search<I, K, V>(
        &mut self,
        payload: HashMap<&str, &str>,
        parameters: I,
    ) -> Result<Vec<Bridgechain>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("bridgechains/search", payload, parameters)
            .await
    }
}
