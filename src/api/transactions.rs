use http::client::Client;
use std::borrow::Borrow;

use api::models::{Response, Transaction, TransactionFees, TransactionTypes};
use error::Error;

pub struct Transactions {
    client: Client,
}

impl Transactions {
    pub fn new(client: Client) -> Transactions {
        Transactions { client }
    }

    pub fn all(&self) -> Result<Response<Vec<Transaction>>, Error> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("transactions", parameters)
    }

    pub fn create<I, K, V>(&self, payload: I) -> Result<Response<Transaction>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post("transactions", Some(payload))
    }

    pub fn show(&self, id: &str) -> Result<Response<Transaction>, Error> {
        let endpoint = format!("transactions/{}", id);
        self.client.get(&endpoint)
    }

    pub fn all_unconfirmed(&self) -> Result<Response<Vec<Transaction>>, Error> {
        self.all_unconfirmed_params(Vec::<(String, String)>::new())
    }

    pub fn all_unconfirmed_params<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("transactions/unconfirmed", parameters)
    }

    pub fn show_unconfirmed(&self, id: &str) -> Result<Response<Vec<Transaction>>, Error> {
        let endpoint = format!("transactions/unconfirmed/{}", id);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(
        &self,
        payload: Option<I>,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("transactions/search", payload, parameters)
    }

    /// Returns the transaction types and their ID
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
    ///   # let client = Connection::new("http://95.179.170.23:4003/api/");
    ///   let types = client.transactions.types().unwrap();
    ///   println!("{}", to_string_pretty(&types).unwrap());
    /// # }
    /// ```
    pub fn types(&self) -> Result<Response<TransactionTypes>, Error> {
        self.client
            .get("transactions/types")
    }

    /// Returns the static fees of the last block processed by the node
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
    ///   let fees = client.transactions.fees().unwrap();
    ///   println!("{}", to_string_pretty(&fees).unwrap());
    /// # }
    /// ```
    pub fn fees(&self) -> Result<Response<TransactionFees>, Error> {
        self.client
            .get("transactions/fees")
    }
}
