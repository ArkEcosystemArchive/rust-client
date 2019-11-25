use crate::api::models::bridgechain::Bridgechain;
use crate::api::models::business::Business;
use crate::api::Result;
use crate::http::client::Client;
use std::borrow::Borrow;
use std::collections::HashMap;

pub struct Businesses {
    client: Client,
}

impl Businesses {
    pub fn new(client: Client) -> Businesses {
        Businesses { client }
    }

    pub fn all(&mut self) -> Result<Vec<Business>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Business>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("businesses", parameters)
    }

    pub fn show(&mut self, ip_addr: &str) -> Result<Business> {
        let endpoint = format!("businesses/{}", ip_addr);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(
        &mut self,
        payload: Option<HashMap<&str, &str>>,
        parameters: I,
    ) -> Result<Vec<Business>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("businesses/search", payload, parameters)
    }

    pub fn bridgechains(&mut self, id: &str) -> Result<Vec<Bridgechain>> {
        self.bridgechains_params(id, Vec::<(String, String)>::new())
    }

    pub fn bridgechains_params<I, K, V>(
        &mut self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Bridgechain>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("businesses/{}/bridgechains", id);
        self.client.get_with_params(&endpoint, parameters)
    }
}
