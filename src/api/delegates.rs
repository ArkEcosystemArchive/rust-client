use http::client::Client;
use std::borrow::Borrow;
use std::collections::HashMap;

use api::models::{Block, Delegate, Wallet};
use api::Result;

pub struct Delegates {
    client: Client,
}

impl Delegates {
    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub fn all(&self) -> Result<Vec<Delegate>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&self, parameters: I) -> Result<Vec<Delegate>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("delegates", parameters)
    }

    pub fn show(&self, id: &str) -> Result<Delegate> {
        let endpoint = format!("delegates/{}", id);
        self.client.get(&endpoint)
    }

    pub fn blocks(&self, id: &str) -> Result<Vec<Block>> {
        self.blocks_params(id, Vec::<(String, String)>::new())
    }

    pub fn blocks_params<I, K, V>(&self, id: &str, parameters: I) -> Result<Vec<Block>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/blocks", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    pub fn voters(&self, id: &str) -> Result<Vec<Wallet>> {
        self.voters_params(id, Vec::<(String, String)>::new())
    }

    pub fn voters_params<I, K, V>(&self, id: &str, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/voters", id);
        self.client.get_with_params(&endpoint, parameters)
    }

    /// Searches the delegates
    ///
    /// # Example
    /// ```
    /// # extern crate serde_json;
    /// # extern crate arkecosystem_client;
    ///
    /// # use serde_json::to_string_pretty;
    /// # use arkecosystem_client::connection::Connection;
    ///
    /// # fn main() {
    ///   # let client = Connection::new("http://167.114.43.38:4003/api/");
    ///   let payload = [("username", "p")].iter();
    ///   let params = [("limit", "2")].iter();
    ///   let search = client.delegates.search(Some(payload), params).unwrap();
    ///   println!("{}", to_string_pretty(&search).unwrap());
    /// # }
    /// ```
    pub fn search<I, K, V>(
        &self,
        payload: Option<HashMap<&str, &str>>,
        parameters: I,
    ) -> Result<Vec<Delegate>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("delegates/search", payload, parameters)
    }
}
