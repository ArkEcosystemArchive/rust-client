use super::*;
use client::{Client, Version};

pub struct Two {
    client: Client,
    pub blocks: Blocks,
    //pub delegates: Delegates,
    //pub loader: Loader,
    //pub peers: Peers,
    //pub signatures: Signatures,
    //pub transactions: Transactions
}

impl Two {

    pub fn new(host: &String) -> Two {
        let client = Client::new(host, Version::Two);
        Two {
            client: client.clone(),
            blocks: Blocks::new(client.clone()),
            //delegates: Delegates::new(client.clone()),
            //loader: Loader::new(client.clone()),
            //peers: Peers::new(client.clone()),
            //signatures: Signatures::new(client.clone()),
            //transactions: Transactions::new(client.clone())
        }
    }

}
