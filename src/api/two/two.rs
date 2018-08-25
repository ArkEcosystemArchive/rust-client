use super::*;
use client::{Client, Version};

pub struct Two {
    client: Client,
    pub blocks: Blocks,
    pub delegates: Delegates,
    pub node: Node,
    pub peers: Peers,
    pub transactions: Transactions,
    pub votes: Votes,
    pub wallets: Wallets
}

impl Two {

    pub fn new(host: &String) -> Two {
        let client = Client::new(host, Version::Two);
        Two {
            client: client.clone(),
            blocks: Blocks::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            node: Node::new(client.clone()),
            peers: Peers::new(client.clone()),
            transactions: Transactions::new(client.clone()),
            votes: Votes::new(client.clone()),
            wallets: Wallets::new(client.clone())
        }
    }

}
