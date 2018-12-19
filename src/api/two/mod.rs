pub mod blocks;
pub mod delegates;
pub mod models;
pub mod node;
pub mod peers;
pub mod transactions;
pub mod votes;
pub mod wallets;

use self::blocks::Blocks;
use self::delegates::Delegates;
use self::node::Node;
use self::peers::Peers;
use self::transactions::Transactions;
use self::votes::Votes;
use self::wallets::Wallets;

use api::{Api, Version};
use http::client::Client;

pub struct Two {
    pub blocks: Blocks,
    pub delegates: Delegates,
    pub node: Node,
    pub peers: Peers,
    pub transactions: Transactions,
    pub votes: Votes,
    pub wallets: Wallets,
    pub client: Client,
}

impl Api for Two {
    fn version() -> Version {
        Version::Two
    }
}

impl Two {
    pub fn new(host: &str) -> Two {
        Two::new_with_client(&Client::new(host))
    }

    pub fn new_with_client(client: &Client) -> Two {
        let mut client = client.clone();
        client.set_version(&Two::version());
        Two {
            blocks: Blocks::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            node: Node::new(client.clone()),
            peers: Peers::new(client.clone()),
            transactions: Transactions::new(client.clone()),
            votes: Votes::new(client.clone()),
            wallets: Wallets::new(client.clone()),
            client
        }
    }
}
