use crate::api::models::lock::Lock;
use crate::api::Result;
use crate::http::client::Client;
use std::borrow::Borrow;
use std::collections::HashMap;

pub struct Locks {
    client: Client,
}

impl Locks {
    pub fn new(client: Client) -> Locks {
        Locks { client }
    }

    pub async fn all(&mut self) -> Result<Vec<Lock>> {
        self.all_params(Vec::<(String, String)>::new()).await
    }

    pub async fn all_params<I, K, V>(&mut self, parameters: I) -> Result<Vec<Lock>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("locks", parameters).await
    }

    pub async fn show(&mut self, lock_id: &str) -> Result<Lock> {
        let endpoint = format!("locks/{}", lock_id);
        self.client.get(&endpoint).await
    }

    pub async fn search<I, K, V>(
        &mut self,
        payload: HashMap<&str, &str>,
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
            .await
    }

    pub async fn unlocked(&mut self, transaction_ids: Vec<&str>) -> Result<Vec<Lock>> {
        let mut payload = HashMap::<&str, Vec<&str>>::new();
        payload.insert("ids", transaction_ids);
        self.client.post("locks/unlocked", payload).await
    }
}
