use failure;
use client::Client;
use serde_json::Value;

pub struct Loader {
    client: Client
}

impl Loader {

    pub fn new(client: Client) -> Loader {
        Loader { client }
    }

    pub fn status(self) -> Result<Value, failure::Error>
    {
        self.client.get("loader/status")
    }

    pub fn sync(self) -> Result<Value, failure::Error>
    {
        self.client.get("loader/status/sync")
    }

    pub fn autoconfigure(self) -> Result<Value, failure::Error>
    {
        self.client.get("loader/autoconfigure")
    }
}
