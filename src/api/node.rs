use api::models::{NodeConfiguration, NodeStatus, NodeSyncing};
use http::client::Client;
use api::Result;

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&self) -> Result<NodeStatus> {
        self.client
            .get("node/status")
    }

    pub fn syncing(&self) -> Result<NodeSyncing> {
        self.client
            .get("node/syncing")
    }

    pub fn configuration(&self) -> Result<NodeConfiguration> {
        self.client
            .get("node/configuration")
    }
}
