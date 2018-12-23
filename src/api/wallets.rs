use http::client::Client;
use std::borrow::Borrow;

use api::models::{Response, Transaction, Wallet};
use error::Error;

pub struct Wallets {
    client: Client,
}

impl Wallets {
    pub fn new(client: Client) -> Wallets {
        Wallets { client }
    }

    pub fn all(&self) -> Result<Response<Vec<Wallet>>, Error> {
        self.all_params(Vec::<(String, String)>::new())
    }

    pub fn all_params<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Wallet>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("wallets", parameters)
    }

    pub fn top(&self) -> Result<Response<Vec<Wallet>>, Error> {
        self.top_params(Vec::<(String, String)>::new())
    }

    pub fn top_params<I, K, V>(
        &self,
        parameters: I,
    ) -> Result<Response<Vec<Wallet>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("wallets/top", parameters)
    }

    pub fn show(&self, id: &str) -> Result<Response<Wallet>, Error> {
        let endpoint = format!("wallets/{}", id);
        self.client.get(&endpoint)
    }

    pub fn transactions(&self, id: &str) -> Result<Response<Vec<Transaction>>, Error> {
        self.transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn transactions_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions", id);
        self.client
            .get_with_params(&endpoint, parameters)
    }

    pub fn sent_transactions(
        &self,
        id: &str,
    ) -> Result<Response<Vec<Transaction>>, Error> {
        self.sent_transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn sent_transactions_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions/sent", id);
        self.client
            .get_with_params(&endpoint, parameters)
    }

    pub fn received_transactions(
        &self,
        id: &str,
    ) -> Result<Response<Vec<Transaction>>, Error> {
        self.received_transactions_params(id, Vec::<(String, String)>::new())
    }

    pub fn received_transactions_params<I, K, V>(
        &self,
        id: &str,
        parameters: I,
    ) -> Result<Response<Vec<Transaction>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("wallets/{}/transactions/received", id);
        self.client
            .get_with_params(&endpoint, parameters)
    }

    pub fn votes(&self, id: &str) -> Result<Response<Vec<Transaction>>, Error> {
        let endpoint = format!("wallets/{}/votes", id);
        self.client.get(&endpoint)
    }

    pub fn search<I, K, V>(&self, parameters: I) -> Result<Response<Vec<Wallet>>, Error>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("wallets/search", parameters)
    }
}
