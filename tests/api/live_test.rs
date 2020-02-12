/// This tests are ignored by default. Run them manually to check for possible differences between
/// local fixtures and actual public REST API returns. All methods/live_test calls MUST pass.
/// Run manually with: `$>cargo test --features network_test`
use arkecosystem_client::Connection;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_peers_all() {
    let mut client = Connection::new(&get_random_seed());
    client.peers.all().await.unwrap();

    let params = [("limit", "20")].iter();
    let peers = client.peers.all_params(params).await.unwrap();
    client.peers.show(peers.data[0].ip.as_str()).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_blocks_all() {
    let mut client = Connection::new(&get_random_seed());
    let blocks = client.blocks.all().await.unwrap();
    client
        .blocks
        .show(blocks.data[0].id.as_str())
        .await
        .unwrap();
    client
        .blocks
        .transactions(blocks.data[0].id.as_str())
        .await
        .unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_bridgechain_all() {
    let mut client = Connection::new(&get_random_seed());
    client.bridgechains.all().await.unwrap();
    // TODO add more - when implemented
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_business() {
    let mut client = Connection::new(&get_random_seed());
    let businesses = client.businesses.all().await.unwrap();
    client
        .businesses
        .bridgechains(businesses.data[0].public_key.as_str())
        .await
        .unwrap();

    // TODO add more - when implemented
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_delegates_all() {
    let mut client = Connection::new(&get_random_seed());
    let actual = client.delegates.all().await.unwrap();
    let params = [("limit", "20")].iter();
    client.delegates.all_params(params).await.unwrap();

    let delegate = client
        .delegates
        .show(actual.data[0].username.as_str())
        .await
        .unwrap();

    client
        .delegates
        .blocks(delegate.data.address.as_str())
        .await
        .unwrap();

    let params = [("limit", "10")].iter();
    client
        .delegates
        .blocks_params(delegate.data.address.as_str(), params)
        .await
        .unwrap();

    client
        .delegates
        .voters(delegate.data.address.as_str())
        .await
        .unwrap();

    let params = [("limit", "4")].iter();
    client
        .delegates
        .voters_params(delegate.data.address.as_str(), params)
        .await
        .unwrap();

    let mut payload = HashMap::new();
    payload.insert("username", "ale");
    client
        .delegates
        .search(payload, [("limit", "20")].iter())
        .await
        .unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_transactions_all() {
    let mut client = Connection::new(&get_random_seed());

    let actual = client.transactions.all().await.unwrap();
    client
        .transactions
        .all_params([("limit", "20")].iter())
        .await
        .unwrap();

    client
        .transactions
        .show(actual.data[0].id.as_str())
        .await
        .unwrap();

    client.transactions.all_unconfirmed().await.unwrap();

    client
        .transactions
        .all_unconfirmed_params([("limit", "20")].iter())
        .await
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
        .await
        .unwrap();

    client.transactions.types().await.unwrap();
    client.transactions.fees().await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_wallets_all() {
    let mut client = Connection::new(&get_random_seed());

    let wallet = client.wallets.all().await.unwrap().data[0].clone();

    client.wallets.show(wallet.address.as_str()).await.unwrap();

    client
        .wallets
        .transactions(wallet.address.as_str())
        .await
        .unwrap();

    client
        .wallets
        .sent_transactions(wallet.address.as_str())
        .await
        .unwrap();

    client
        .wallets
        .received_transactions(wallet.address.as_str())
        .await
        .unwrap();

    client.wallets.votes(wallet.address.as_str()).await.unwrap();

    let mut query = HashMap::new();
    query.insert("address", wallet.address.as_str());

    client
        .wallets
        .search(query, [("limit", "20")].iter())
        .await
        .unwrap();

    client.wallets.top().await.unwrap();
    client.wallets.locks(wallet.address.as_str()).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_votes_all() {
    let mut client = Connection::new(&get_random_seed());

    let transaction = client.votes.all().await.unwrap().data[0].clone();
    client
        .votes
        .all_params([("limit", "20")].iter())
        .await
        .unwrap();
    client.votes.show(transaction.id.as_str()).await.unwrap();
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_locks_all() {
    let mut client = Connection::new(&get_random_seed());

    if !client.locks.all().await.unwrap().data.is_empty() {
        let lock_data = client.locks.all().await.unwrap().data[0].clone();

        client
            .locks
            .all_params([("senderPublicKey", lock_data.sender_public_key.as_str())].iter())
            .await
            .unwrap();

        client.locks.show(&lock_data.lock_id).await.unwrap();

        let mut query = HashMap::new();
        query.insert("recipientId", lock_data.recipient_id.as_str());

        client
            .locks
            .search(query, [("limit", "20")].iter())
            .await
            .unwrap();

        let mut trx_ids = Vec::new();
        trx_ids.push(lock_data.lock_id.as_str());

        client.locks.unlocked(trx_ids).await.unwrap();
    }
}

#[tokio::test]
#[cfg_attr(not(feature = "network_test"), ignore)]
async fn test_live_node_all() {
    let mut client = Connection::new(&get_random_seed());

    client.node.status().await.unwrap();

    client.node.syncing().await.unwrap();

    client.node.configuration().await.unwrap();

    client.node.fees([("days", "20")].iter()).await.unwrap();
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
