use api::models::{NodeConfiguration, NodeStatus, NodeSyncing, NodeFees};
use api::Result;
use http::client::Client;

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&self) -> Result<NodeStatus> {
        self.client.get("node/status")
    }

    pub fn syncing(&self) -> Result<NodeSyncing> {
        self.client.get("node/syncing")
    }

    pub fn configuration(&self) -> Result<NodeConfiguration> {
        self.client.get("node/configuration")
    }

    pub fn fees(&self) -> Result<NodeFees> {
        self.client.get("node/fees")
    }
}
