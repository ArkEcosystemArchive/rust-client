/// This tests are ignored by default. Run them manually to check for possible differences between
/// local fixtures and actual public REST API returns. All methods/live_test calls MUST pass.
/// Run manually with: `$>cargo test --features network_test`
use arkecosystem_client::Connection;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_peers_all() {
    let mut client = Connection::new(&get_random_seed());
    client.peers.all().unwrap();

    let params = [("limit", "20")].iter();
    let peers = client.peers.all_params(params).unwrap();
    client.peers.show(peers.data[0].ip.as_str()).unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_blocks_all() {
    let mut client = Connection::new(&get_random_seed());
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
    let mut client = Connection::new(&get_random_seed());
    client.bridgechains.all().unwrap();
    // TODO add more - when implemented
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_business() {
    let mut client = Connection::new(&get_random_seed());
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
    let mut client = Connection::new(&get_random_seed());
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
    let mut client = Connection::new(&get_random_seed());

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
    let mut client = Connection::new(&get_random_seed());

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
    let mut client = Connection::new(&get_random_seed());

    let transaction = client.votes.all().unwrap().data[0].clone();
    client.votes.all_params([("limit", "20")].iter()).unwrap();
    client.votes.show(transaction.id.as_str()).unwrap();
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_locks_all() {
    let mut client = Connection::new(&get_random_seed());

    if !client.locks.all().unwrap().data.is_empty() {
        let lock_data = client.locks.all().unwrap().data[0].clone();

        client
            .locks
            .all_params([("senderPublicKey", lock_data.sender_public_key.as_str())].iter())
            .unwrap();

        client.locks.show(&lock_data.lock_id).unwrap();

        let mut query = HashMap::new();
        query.insert("recipientId", lock_data.recipient_id.as_str());

        client
            .locks
            .search(query, [("limit", "20")].iter())
            .unwrap();

        let mut trx_ids = Vec::new();
        trx_ids.push(lock_data.lock_id.as_str());

        client.locks.unlocked(trx_ids).unwrap();
    }
}

#[test]
#[cfg_attr(not(feature = "network_test"), ignore)]
fn test_live_node_all() {
    let mut client = Connection::new(&get_random_seed());

    client.node.status().unwrap();

    client.node.syncing().unwrap();

    client.node.configuration().unwrap();

    client.node.fees([("days", "20")].iter()).unwrap();
}

fn get_random_seed() -> String {
    let seeds = vec![
        "167.114.29.51",
        "167.114.29.52",
        "167.114.29.53",
        "167.114.29.54",
        "167.114.29.55",
    ];

    format!(
        "http://{}:4003/api/",
        seeds.choose(&mut rand::thread_rng()).unwrap(),
    )
}
