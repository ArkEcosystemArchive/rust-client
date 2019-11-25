/// This test is ignored by default. It is to bo run manual to check possible differences between
/// local fixtures and public returns. If all serde method pass - it should be ok.
/// It servers as additional test check for serde/struct compatibility on a live running node.
/// RUN ONLY MANUALLY with `$>cargo test --features network_test`
use arkecosystem_client::Connection;
use std::collections::HashMap;

// const LIVE_HOST: &str = "http://185.170.115.40:4003/api/";
const LIVE_HOST: &str = "https://dexplorer.ark.io/api/";

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_peers_all() {
    let mut client = Connection::new(LIVE_HOST);
    client.peers.all().unwrap();

    let params = [("limit", "20")].iter();
    let peers = client.peers.all_params(params).unwrap();
    client.peers.show(peers.data[0].ip.as_str()).unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_blocks_all() {
    let mut client = Connection::new(LIVE_HOST);
    let blocks = client.blocks.all().unwrap();
    client.blocks.show(blocks.data[0].id.as_str()).unwrap();
    client
        .blocks
        .transactions(blocks.data[0].id.as_str())
        .unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_bridgechain_all() {
    let mut client = Connection::new(LIVE_HOST);
    client.bridgechains.all().unwrap();
    // TODO add more - when implemented
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_business() {
    let mut client = Connection::new(LIVE_HOST);
    let businesses = client.businesses.all().unwrap();
    client
        .businesses
        .bridgechains(businesses.data[0].public_key.as_str())
        .unwrap();

    // TODO add more - when implemented
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_delegates_all() {
    let mut client = Connection::new(LIVE_HOST);
    let actual = client.delegates.all().unwrap();
    let params = [("limit", "20")].iter();
    client.delegates.all_params(params).unwrap();

    let delegate = client
        .delegates
        .show(actual.data[0].username.as_str())
        .unwrap();

    client
        .delegates
        .blocks(delegate.data.address.as_str())
        .unwrap();

    let params = [("limit", "10")].iter();
    client
        .delegates
        .blocks_params(delegate.data.address.as_str(), params)
        .unwrap();

    client
        .delegates
        .voters(delegate.data.address.as_str())
        .unwrap();

    let params = [("limit", "4")].iter();
    client
        .delegates
        .voters_params(delegate.data.address.as_str(), params)
        .unwrap();

    let mut payload = HashMap::new();
    payload.insert("username", "ale");
    client
        .delegates
        .search(payload, [("limit", "20")].iter())
        .unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_transactions_all() {
    let mut client = Connection::new(LIVE_HOST);

    let actual = client.transactions.all().unwrap();
    client
        .transactions
        .all_params([("limit", "20")].iter())
        .unwrap();

    client
        .transactions
        .show(actual.data[0].id.as_str())
        .unwrap();

    client.transactions.all_unconfirmed().unwrap();

    client
        .transactions
        .all_unconfirmed_params([("limit", "20")].iter())
        .unwrap();

    //    client
    //        .transactions
    //        .show_unconfirmed(actual.data[0].id.as_str())
    //        .unwrap();

    let mut query = HashMap::new();
    query.insert("senderId", actual.data[0].sender.as_str());
    client
        .transactions
        .search(query, [("limit", "20")].iter())
        .unwrap();

    client.transactions.types().unwrap();
    client.transactions.fees().unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_wallets_all() {
    let mut client = Connection::new(LIVE_HOST);

    let wallet = client.wallets.all().unwrap().data[0].clone();

    client.wallets.show(wallet.address.as_str()).unwrap();

    client
        .wallets
        .transactions(wallet.address.as_str())
        .unwrap();

    client
        .wallets
        .sent_transactions(wallet.address.as_str())
        .unwrap();

    client
        .wallets
        .received_transactions(wallet.address.as_str())
        .unwrap();

    client.wallets.votes(wallet.address.as_str()).unwrap();

    let mut query = HashMap::new();
    query.insert("address", wallet.address.as_str());

    client
        .wallets
        .search(query, [("limit", "20")].iter())
        .unwrap();

    client.wallets.top().unwrap();
    client.wallets.locks(wallet.address.as_str()).unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_votes_all() {
    let mut client = Connection::new(LIVE_HOST);

    let transaction = client.votes.all().unwrap().data[0].clone();
    client.votes.all_params([("limit", "20")].iter()).unwrap();
    client.votes.show(transaction.id.as_str()).unwrap();
}
