use crate::api::models::fee::FeeStats;
use crate::api::models::node::{NodeConfiguration, NodeStatus, NodeSyncing};
use crate::api::Result;
use crate::http::client::Client;
use std::borrow::Borrow;

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&mut self) -> Result<NodeStatus> {
        self.client.get("node/status")
    }

    pub fn syncing(&mut self) -> Result<NodeSyncing> {
        self.client.get("node/syncing")
    }

    pub fn configuration(&mut self) -> Result<NodeConfiguration> {
        self.client.get("node/configuration")
    }

    pub fn fees<I, K, V>(&mut self, parameters: I) -> Result<Vec<FeeStats>>
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.client.get_with_params("node/fees", parameters)
    }
}
