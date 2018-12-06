use super::*;
use api::{Api, Version};
use http::client::Client;

pub struct One {
    pub accounts: Accounts,
    pub blocks: Blocks,
    pub delegates: Delegates,
    pub loader: Loader,
    pub peers: Peers,
    pub signatures: Signatures,
    pub transactions: Transactions,
}

impl Api for One {
    fn version() -> Version {
        Version::One
    }
}

impl One {
    pub fn new(client: &Client) -> One {
        let mut client = client.clone();
        client.set_version(One::version());
        One {
            accounts: Accounts::new(client.clone()),
            blocks: Blocks::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            loader: Loader::new(client.clone()),
            peers: Peers::new(client.clone()),
            signatures: Signatures::new(client.clone()),
            transactions: Transactions::new(client.clone()),
        }
    }
}
