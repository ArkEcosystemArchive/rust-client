use failure;
use client::Client;
use serde_json::Value;

pub struct Node {
    client: Client
}

impl Node {

    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(self) -> Result<Value, failure::Error> {
        self.client.get("node/status")
    }

    pub fn syncing(self) -> Result<Value, failure::Error> {
        self.client.get("node/syncing")
    }

    pub fn configuration(self) -> Result<Value, failure::Error> {
        self.client.get("node/configuration")
    }
}
