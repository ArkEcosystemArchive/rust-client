use super::*;
use client::{Client, Version};

pub struct One {
    client: Client,
    pub accounts: Accounts,
    pub blocks: Blocks,
    pub delegates: Delegates,
    pub loader: Loader
}

impl One {

    pub fn new(host: String) -> One {
        let client = Client::new(host, Version::One);
        One {
            client: client.clone(),
            accounts: Accounts::new(client.clone()),
            blocks: Blocks::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            loader: Loader::new(client.clone())
        }
    }

}
