use std::str::FromStr;

use serde_json::Value;

use arkecosystem_client::api::models::block::Block;
use arkecosystem_client::api::models::delegate::Delegate;
use arkecosystem_client::api::models::fee::FeeSchema;
use arkecosystem_client::api::models::peer::Peer;
use arkecosystem_client::api::models::shared::Meta;
use arkecosystem_client::api::models::timestamp::Timestamp;
use arkecosystem_client::api::models::transaction::Transaction;
use arkecosystem_client::api::models::wallet::Wallet;

pub fn assert_meta(actual: Meta, expected: &Value) {
    assert_eq!(actual.count, expected["count"].as_u64().unwrap() as u32);
    assert_eq!(
        actual.page_count,
        expected["pageCount"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.total_count,
        expected["totalCount"].as_u64().unwrap() as u32
    );
    if actual.next.is_some() {
        assert_eq!(actual.next.unwrap(), expected["next"].as_str().unwrap());
    }
    if actual.previous.is_some() {
        assert_eq!(
            actual.previous.unwrap(),
            expected["previous"].as_str().unwrap()
        );
    }
    assert_eq!(actual.self_url, expected["self"].as_str().unwrap());
    assert_eq!(actual.first, expected["first"].as_str().unwrap());
    if actual.last.is_some() {
        assert_eq!(actual.last.unwrap(), expected["last"].as_str().unwrap());
    }
}

pub fn assert_timestamp_data(actual: &Timestamp, expected: &Value) {
    assert_eq!(actual.epoch, expected["epoch"].as_u64().unwrap() as u32);
    assert_eq!(actual.unix, expected["unix"].as_u64().unwrap() as u32);
    assert_eq!(actual.human, expected["human"].as_str().unwrap());
}

pub fn assert_block_data(actual: &Block, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    assert_eq!(actual.version, expected["version"].as_u64().unwrap() as u8);
    assert_eq!(actual.height, expected["height"].as_u64().unwrap());
    assert_eq!(actual.previous, expected["previous"].as_str().unwrap());
    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());

    assert_eq!(
        actual.forged.reward,
        u64::from_str(expected["forged"]["reward"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.fee,
        u64::from_str(expected["forged"]["fee"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.total,
        u64::from_str(expected["forged"]["total"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.amount,
        u64::from_str(expected["forged"]["amount"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.payload.hash,
        expected["payload"]["hash"].as_str().unwrap()
    );
    assert_eq!(
        actual.payload.length,
        expected["payload"]["length"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.generator.username,
        expected["generator"]["username"].as_str().unwrap()
    );
    assert_eq!(
        actual.generator.address,
        expected["generator"]["address"].as_str().unwrap()
    );
    assert_eq!(
        actual.generator.public_key,
        expected["generator"]["publicKey"].as_str().unwrap()
    );
    assert_eq!(
        actual.transactions,
        expected["transactions"].as_u64().unwrap() as u32
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
}

pub fn assert_transaction_data(actual: Transaction, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    assert_eq!(actual.block_id, expected["blockId"].as_str().unwrap());
    if let Some(version) = actual.version {
        assert_eq!(version, expected["version"].as_u64().unwrap() as u16);
    }
    assert_eq!(
        actual.transaction_type as u64,
        expected["type"].as_u64().unwrap()
    );
    assert_eq!(
        actual.type_group as u64,
        expected["typeGroup"].as_u64().unwrap()
    );

    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        actual.fee,
        u64::from_str(expected["fee"].as_str().unwrap()).unwrap()
    );

    assert_eq!(actual.sender, expected["sender"].as_str().unwrap());
    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );

    if let Some(recipient) = actual.recipient {
        assert_eq!(recipient, expected["recipient"].as_str().unwrap());
    }
    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());
    if let Some(sign_signature) = actual.sign_signature {
        assert_eq!(sign_signature, expected["signSignature"].as_str().unwrap());
    }
    if let Some(vendor_field) = actual.vendor_field {
        assert_eq!(vendor_field, expected["vendorField"].as_str().unwrap());
    }

    // TODO: asset should be tested on each transaction type

    assert_eq!(
        actual.confirmations,
        expected["confirmations"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
    assert_eq!(
        actual.nonce,
        u64::from_str(expected["nonce"].as_str().unwrap()).unwrap()
    );
}

pub fn assert_wallet_data(actual: Wallet, expected: &Value) {
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    if let Some(public_key) = actual.public_key {
        assert_eq!(public_key, expected["publicKey"].as_str().unwrap());
    }
    if let Some(username) = actual.username {
        assert_eq!(username, expected["username"].as_str().unwrap());
    }
    if let Some(second_public_key) = actual.second_public_key {
        assert_eq!(
            second_public_key,
            expected["secondPublicKey"].as_str().unwrap()
        );
    }
    assert_eq!(actual.balance, expected["balance"].as_u64().unwrap());
    assert_eq!(
        actual.is_delegate,
        expected["isDelegate"].as_bool().unwrap()
    );
}

pub fn assert_configuration_fees(actual: &FeeSchema, expected: &Value) {
    assert_eq!(actual.transfer, expected["transfer"].as_u64().unwrap());
    assert_eq!(
        actual.second_signature,
        expected["secondSignature"].as_u64().unwrap()
    );
    assert_eq!(
        actual.delegate_registration,
        expected["delegateRegistration"].as_u64().unwrap()
    );
    assert_eq!(actual.vote, expected["vote"].as_u64().unwrap());
    assert_eq!(
        actual.multi_signature,
        expected["multiSignature"].as_u64().unwrap()
    );
    assert_eq!(actual.ipfs, expected["ipfs"].as_u64().unwrap());
    assert_eq!(
        actual.multi_payment,
        expected["multiPayment"].as_u64().unwrap()
    );
    assert_eq!(
        actual.delegate_resignation,
        expected["delegateResignation"].as_u64().unwrap()
    );
}

pub fn assert_delegate_data(actual: Delegate, expected: &Value) {
    assert_eq!(actual.username, expected["username"].as_str().unwrap());
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    assert_eq!(actual.public_key, expected["publicKey"].as_str().unwrap());
    assert_eq!(actual.votes, expected["votes"].as_str().unwrap());
    assert_eq!(actual.rank, expected["rank"].as_u64().unwrap() as u32);
    assert_eq!(
        actual.blocks.produced,
        expected["blocks"]["produced"].as_u64().unwrap()
    );

    if actual.blocks.last.is_some() {
        let last = actual.blocks.last.unwrap().clone();
        assert_eq!(last.id, expected["blocks"]["last"]["id"].as_str().unwrap());
        assert_eq!(last.height, expected["blocks"]["last"]["height"].as_u64().unwrap());

        assert_timestamp_data(
            &last.timestamp,
            &expected["blocks"]["last"]["timestamp"].clone(),
        );
    }
    assert_eq!(
        actual.forged.rewards,
        u64::from_str(expected["forged"]["rewards"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.fees,
        u64::from_str(expected["forged"]["fees"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.total,
        u64::from_str(expected["forged"]["total"].as_str().unwrap()).unwrap()
    );
}

pub fn assert_peer_data(actual: &Peer, expected: &Value) {
    assert_eq!(actual.ip, expected["ip"].as_str().unwrap());
    assert_eq!(actual.port, expected["port"].as_u64().unwrap() as u16);
    assert_eq!(actual.version, expected["version"].as_str().unwrap());
    assert_eq!(actual.height, expected["height"].as_u64().unwrap());
    assert_eq!(actual.status, expected["status"].as_u64().unwrap() as u16);
    assert_eq!(actual.os, expected["os"].as_str().unwrap());
    assert_eq!(actual.latency, expected["latency"].as_u64().unwrap() as u32);
    assert_eq!(actual.hashid, expected["hashid"].as_str().unwrap());
}

