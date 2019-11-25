use crate::api::models::transaction::{
    Transaction, TransactionFees, TransactionPostResponse, TransactionTypes,
};
use crate::api::Result;
use crate::http::client::Client;
use std::borrow::Borrow;
use std::collections::HashMap;

pub struct Transactions {
    client: Client,
}

impl Transactions {
    pub fn new(client: Client) -> Transactions {
        Transactions { client }
    }

    pub fn all(&mut self) -> Result<Vec<Transaction>> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("transactions", parameters)
    }

    pub fn create(&mut self, transactions: Vec<&str>) -> Result<TransactionPostResponse> {
        let mut payload = HashMap::<&str, Vec<&str>>::new();
        payload.insert("transactions", transactions);
        eprintln!("payload = {:#?}", payload);
        self.client.post("transactions", payload)
    }

    pub fn show(&mut self, id: &str) -> Result<Transaction> {
        let endpoint = format!("transactions/{}", id);
        self.client.get(&endpoint)
    }

    pub fn all_unconfirmed(&mut self) -> Result<Vec<Transaction>> {
        self.all_unconfirmed_params(Vec::<(String, String)>::new())
    }

    pub fn all_unconfirmed_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("transactions/unconfirmed", parameters)
    }

    pub fn show_unconfirmed(&mut self, id: &str) -> Result<Transaction> {
        let endpoint = format!("transactions/unconfirmed/{}", id);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(
        &mut self,
        payload: HashMap<&str, &str>,
        parameters: I,
    ) -> Result<Vec<Transaction>>
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
    /// use serde_json::to_string_pretty;
    /// use arkecosystem_client::connection::Connection;
    ///
    /// let client = Connection::new("http://95.179.170.23:4003/api/");
    /// let types = client.transactions.types().unwrap();
    /// println!("{}", to_string_pretty(&types).unwrap());
    /// ```
    pub fn types(&mut self) -> Result<TransactionTypes> {
        self.client.get("transactions/types")
    }

    /// Returns the static fees of the last block processed by the node
    ///
    /// # Example
    /// ```
    /// use serde_json::to_string_pretty;
    /// use arkecosystem_client::connection::Connection;
    ///
    /// let client = Connection::new("http://167.114.43.38:4003/api/");
    /// let fees = client.transactions.fees().unwrap();
    /// println!("{}", to_string_pretty(&fees).unwrap());
    /// ```
    pub fn fees(&mut self) -> Result<TransactionFees> {
        self.client.get("transactions/fees")
    }
}
