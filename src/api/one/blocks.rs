use failure;
use client::Client;
use std::borrow::Borrow;

pub struct Blocks {
    client: Client
}

impl Blocks {

    pub fn new(client: Client) -> Blocks {
        Blocks { client }
    }

    pub fn all<I, K, V>(self, parameters: I) -> Result<String, failure::Error>
        where I: IntoIterator,
                 I::Item: Borrow<(K, V)>,
                 K: AsRef<str>,
                 V: AsRef<str>
    {
        self.client.get_with_params("blocks", parameters)
    }

    pub fn show(self, id: String) -> Result<String, failure::Error> {
        self.client.get_with_params("blocks/get", &[("id", &id)])
    }

    pub fn epoch(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getEpoch")
    }

    pub fn height(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getHeight")
    }

    pub fn nethash(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getNethash")
    }

    pub fn fee(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getFee")
    }

    pub fn fees(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getFees")
    }

    pub fn milestone(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getMilestone")
    }

    pub fn reward(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getReward")
    }

    pub fn supply(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getSupply")
    }

    pub fn status(self) -> Result<String, failure::Error> {
        self.client.get("blocks/getStatus")
    }
}
