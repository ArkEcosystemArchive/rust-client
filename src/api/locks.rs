use std::borrow::Borrow;

use crate::api::models::lock::Lock;
use crate::api::Result;
use crate::http::client::Client;
use std::collections::HashMap;

pub struct Locks {
    client: Client,
}

impl Locks {
    pub fn new(client: Client) -> Locks {
        Locks { client }
    }

    pub fn all(&mut self) -> Result<Vec<Lock>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Lock>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("locks", parameters)
    }

    pub fn show(&mut self, ip_addr: &str) -> Result<Lock> {
        let endpoint = format!("locks/{}", ip_addr);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(
        &mut self,
        payload: Option<HashMap<&str, &str>>,
        parameters: I,
    ) -> Result<Vec<Lock>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("locks/search", payload, parameters)
    }

    pub fn unlocked<I, K, V>(
        &mut self,
        payload: Option<HashMap<&str, &str>>,
        parameters: I,
    ) -> Result<Vec<Lock>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("locks/unlocked", payload, parameters)
    }
}
