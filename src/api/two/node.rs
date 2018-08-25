extern crate failure;

use client::Client;

pub struct Node {
    client: Client
}

impl Node {

    pub fn new(client: Client) -> Node {
        Node { client }
    }

    pub fn status(self) -> Result<String, failure::Error> {
        self.client.get("node/status")
    }

    pub fn syncing(self) -> Result<String, failure::Error> {
        self.client.get("node/syncing")
    }

    pub fn configuration(self) -> Result<String, failure::Error> {
        self.client.get("node/configuration")
    }
}
