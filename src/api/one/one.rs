use super::Accounts;
use client::{Client, Version};

pub struct One {
    client: Client,
    pub accounts: Accounts

}

impl One {

    pub fn new(host: String) -> One {
        let client = Client::new(host, Version::One);
        One {
            client: client.clone(),
            accounts: Accounts::new(client.clone())
        }
    }

}
