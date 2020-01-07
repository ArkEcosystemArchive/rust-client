use crate::http::client::Client;
use std::borrow::Borrow;

use crate::api::models::lock::Lock;
use crate::api::models::transaction::Transaction;
use crate::api::models::wallet::Wallet;
use crate::api::Result;
use std::collections::HashMap;

pub struct Wallets {
    client: Client,
}

impl Wallets {
    pub fn new(client: Client) -> Wallets {
        Wallets { client }
    }

    pub async fn all(&mut self) -> Result<Vec<Wallet>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("api/wallets", parameters).await
    }

    pub async fn top(&mut self) -> Result<Vec<Wallet>> {
        self.top_params(Vec::<(String, String)>::new()).await
    }

    pub async fn top_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .get_with_params("api/wallets/top", parameters)
            .await
    }

    pub async fn show(&mut self, id: &str) -> Result<Wallet> {
        let endpoint = format!("api/wallets/{}", id);
        self.client.get(&endpoint).await
    }

    pub async fn transactions(&mut self, id: &str) -> Result<Vec<Transaction>> {
        self.transactions_params(id, Vec::<(String, String)>::new())
            .await
    }

    pub async fn transactions_params<I, K, V>(
        &mut self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("api/wallets/{}/transactions", id);
        self.client.get_with_params(&endpoint, parameters).await
    }

    pub async fn sent_transactions(&mut self, id: &str) -> Result<Vec<Transaction>> {
        self.sent_transactions_params(id, Vec::<(String, String)>::new())
            .await
    }

    pub async fn sent_transactions_params<I, K, V>(
        &mut self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("api/wallets/{}/transactions/sent", id);
        self.client.get_with_params(&endpoint, parameters).await
    }

    pub async fn received_transactions(&mut self, id: &str) -> Result<Vec<Transaction>> {
        self.received_transactions_params(id, Vec::<(String, String)>::new())
            .await
    }

    pub async fn received_transactions_params<I, K, V>(
        &mut self,
        id: &str,
        parameters: I,
    ) -> Result<Vec<Transaction>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let endpoint = format!("api/wallets/{}/transactions/received", id);
        self.client.get_with_params(&endpoint, parameters).await
    }

    pub async fn votes(&mut self, id: &str) -> Result<Vec<Transaction>> {
        let endpoint = format!("api/wallets/{}/votes", id);
        self.client.get(&endpoint).await
    }

    pub async fn search<I, K, V>(
        &mut self,
        payload: HashMap<&str, &str>,
        parameters: I,
    ) -> Result<Vec<Wallet>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client
            .post_with_params("api/wallets/search", payload, parameters)
            .await
    }

    pub async fn locks(&mut self, id: &str) -> Result<Vec<Lock>> {
        let endpoint = format!("api/wallets/{}/locks", id);
        self.client.get(&endpoint).await
    }
}
