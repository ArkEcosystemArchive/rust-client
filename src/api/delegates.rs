use std::borrow::Borrow;
use std::collections::HashMap;

use crate::api::models::block::Block;
use crate::api::models::delegate::Delegate;
use crate::api::models::wallet::{Balances, Wallet};
use crate::api::Result;
use crate::http::client::Client;

pub struct Delegates {
    client: Client,
}

impl Delegates {
    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub async fn all(&mut self) -> Result<Vec<Delegate>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Delegate>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("delegates", parameters).await
    }

    pub async fn show(&mut self, id: &str) -> Result<Delegate> {
        let endpoint = format!("delegates/{}", id);
        self.client.get(&endpoint).await
    }

    pub async fn blocks(&mut self, id: &str) -> Result<Vec<Block>> {
        self.blocks_params(id, Vec::<(String, String)>::new()).await
    }

    pub async fn blocks_params<I, K, V>(&mut self, id: &str, parameters: I) -> Result<Vec<Block>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/blocks", id);
        self.client.get_with_params(&endpoint, parameters).await
    }

    pub async fn voters(&mut self, id: &str) -> Result<Vec<Wallet>> {
        self.voters_params(id, Vec::<(String, String)>::new()).await
    }

    pub async fn voters_params<I, K, V>(&mut self, id: &str, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/voters", id);
        self.client.get_with_params(&endpoint, parameters).await
    }

    /// Returns the voters of a delegate and their balances
    ///
    /// # Example
    /// ```
    /// use serde_json::to_string_pretty;
    /// use arkecosystem_client::connection::Connection;
    ///
    /// let client = Connection::new("http://167.114.43.38:4003/api/");
    /// let delegate_id = "yo";
    /// let voters_balances = client.delegates.voters_balances(&delegate_id).unwrap();
    /// println!("{}", to_string_pretty(&voters_balances).unwrap());
    /// ```
    pub async fn voters_balances(&mut self, id: &str) -> Result<Balances> {
        let endpoint = format!("delegates/{}/voters/balances", id);
        self.client.get(&endpoint).await
    }

    /// Searches the delegates
    ///
    /// # Example
    /// ```
    /// use serde_json::to_string_pretty;
    /// use arkecosystem_client::connection::Connection;
    ///
    /// let client = Connection::new("http://167.114.43.38:4003/api/");
    /// let payload = [("username", "p")].iter();
    /// let params = [("limit", "2")].iter();
    /// let search = client.delegates.search(Some(payload), params).unwrap();
    /// println!("{}", to_string_pretty(&search).unwrap());
    /// ```
    pub async fn search<I, K, V>(
        &mut self,
        payload: HashMap<&str, &str>,
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
            .await
    }
}
