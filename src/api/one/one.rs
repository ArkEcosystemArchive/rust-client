use super::*;
use http::client::Client;
use connection::Version;

pub struct One {
    client: Client,
    pub accounts: Accounts,
    pub blocks: Blocks,
    pub delegates: Delegates,
    pub loader: Loader,
    pub peers: Peers,
    pub signatures: Signatures,
    pub transactions: Transactions
}

impl One {

    pub fn new(host: &String) -> One {
        let client = Client::new(host, Version::One);
        One {
            client: client.clone(),
            accounts: Accounts::new(client.clone()),
            blocks: Blocks::new(client.clone()),
            delegates: Delegates::new(client.clone()),
            loader: Loader::new(client.clone()),
            peers: Peers::new(client.clone()),
            signatures: Signatures::new(client.clone()),
            transactions: Transactions::new(client.clone())
        }
    }

}
