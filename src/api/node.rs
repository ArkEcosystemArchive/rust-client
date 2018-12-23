use api::models::{NodeConfiguration, NodeStatus, NodeSyncing, Response};
use http::client::Client;
use error::Error;

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&self) -> Result<Response<NodeStatus>, Error> {
        self.client
            .get("node/status")
    }

    pub fn syncing(&self) -> Result<Response<NodeSyncing>, Error> {
        self.client
            .get("node/syncing")
    }

    pub fn configuration(&self) -> Result<Response<NodeConfiguration>, Error> {
        self.client
            .get("node/configuration")
    }
}
