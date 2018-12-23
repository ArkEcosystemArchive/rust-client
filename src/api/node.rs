use api::models::{NodeConfiguration, NodeStatus, NodeSyncing, Response};
use http::client::Client;
use api::ApiResult;

pub struct Node {
    client: Client,
}

impl Node {
    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(&self) -> ApiResult<Response<NodeStatus>> {
        self.client
            .get("node/status")
    }

    pub fn syncing(&self) -> ApiResult<Response<NodeSyncing>> {
        self.client
            .get("node/syncing")
    }

    pub fn configuration(&self) -> ApiResult<Response<NodeConfiguration>> {
        self.client
            .get("node/configuration")
    }
}
