use http::client::Client;
use std::borrow::Borrow;

use api::models::{Balances, Block, Delegate, Response, Wallet};
use api::ApiResult;

pub struct Delegates {
    client: Client,
}

impl Delegates {
    pub fn new(client: Client) -> Delegates {
        Delegates { client }
    }

    pub fn all(&self) -> ApiResult<Response<Vec<Delegate>>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(
        &self,
        parameters: I,
    ) -> ApiResult<Response<Vec<Delegate>>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("delegates", parameters)
    }

    pub fn show(&self, id: &str) -> ApiResult<Response<Delegate>> {
        let endpoint = format!("delegates/{}", id);
        self.client.get(&endpoint)
    }

    pub fn blocks(&self, id: &str) -> ApiResult<Response<Vec<Block>>> {
        self.blocks_params(id, Vec::<(String, String)>::new())
    }

    pub fn blocks_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> ApiResult<Response<Vec<Block>>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/blocks", id);
        self.client
            .get_with_params(&endpoint, parameters)
    }

    pub fn voters(&self, id: &str) -> ApiResult<Response<Vec<Wallet>>> {
        self.voters_params(id, Vec::<(String, String)>::new())
    }

    pub fn voters_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> ApiResult<Response<Vec<Wallet>>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("delegates/{}/voters", id);
        self.client
            .get_with_params(&endpoint, parameters)
    }

    /// Returns the voters of a delegate and their balances
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
    ///   let delegate_id = "yo";
    ///   let voters_balances = client.delegates.voters_balances(&delegate_id).unwrap();
    ///   println!("{}", to_string_pretty(&voters_balances).unwrap());
    /// # }
    /// ```
    pub fn voters_balances(&self, id: &str) -> ApiResult<Response<Balances>> {
        let endpoint = format!("delegates/{}/voters/balances", id);
        self.client.get(&endpoint)
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
        payload: Option<I>,
        parameters: I,
    ) -> ApiResult<Response<Vec<Delegate>>>
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
