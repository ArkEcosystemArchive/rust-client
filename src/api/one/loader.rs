extern crate failure;

use client::Client;

pub struct Loader {
    client: Client
}

impl Loader {

    pub fn new(client: Client) -> Loader {
        Loader { client }
    }

    pub fn status(self) -> Result<String, failure::Error>
    {
        self.client.get("loader/status")
    }

    pub fn sync(self) -> Result<String, failure::Error>
    {
        self.client.get("loader/status/sync")
    }

    pub fn autoconfigure(self) -> Result<String, failure::Error>
    {
        self.client.get("loader/autoconfigure")
    }
}
