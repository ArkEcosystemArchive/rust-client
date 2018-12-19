use failure;
use serde_json::from_value;

use http::client::Client;
use api::models::{NodeConfiguration, NodeStatus, NodeSyncing, Response};

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&self) -> Result<Response<NodeStatus>, failure::Error> {
        self.client
            .get("node/status")
            .map(|v| from_value(v).unwrap())
    }

    pub fn syncing(&self) -> Result<Response<NodeSyncing>, failure::Error> {
        self.client
            .get("node/syncing")
            .map(|v| from_value(v).unwrap())
    }

    pub fn configuration(&self) -> Result<Response<NodeConfiguration>, failure::Error> {
        self.client
            .get("node/configuration")
            .map(|v| from_value(v).unwrap())
    }
}
