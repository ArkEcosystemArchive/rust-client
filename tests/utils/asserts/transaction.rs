use crate::utils::asserts::shared::assert_timestamp_data;
use arkecosystem_client::api::models::lock::Lock;
use arkecosystem_client::api::models::transaction::{Transaction, TransactionPostResponse};
use serde_json::Value;
use std::borrow::Borrow;
use std::str::FromStr;

pub fn assert_transaction_data(actual: Transaction, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    if let Some(block_id) = actual.block_id {
        assert_eq!(block_id, expected["blockId"].as_str().unwrap());
    }
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

    assert_eq!(
        actual.confirmations,
        expected["confirmations"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
    if let Some(nonce) = actual.nonce {
        assert_eq!(nonce, expected["nonce"].as_str().unwrap());
    }
}

pub fn assert_transaction_post_data(actual: TransactionPostResponse, expected: &Value) {
    for (pos, value) in actual.accept.iter().enumerate() {
        assert_eq!(value, &expected["accept"][pos]);
    }
    for (pos, value) in actual.broadcast.iter().enumerate() {
        assert_eq!(value, &expected["broadcast"][pos]);
    }
    for (pos, value) in actual.excess.iter().enumerate() {
        assert_eq!(value, &expected["excess"][pos]);
    }
    for (pos, value) in actual.invalid.iter().enumerate() {
        assert_eq!(value, &expected["invalid"][pos]);
    }
}

pub fn assert_vote_data(actual: Transaction, expected: &Value) {
    assert_transaction_data(actual, &expected);
}

pub fn assert_lock_data(actual: Lock, expected: &Value) {
    assert_eq!(actual.lock_id, expected["lockId"].as_str().unwrap());
    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );
    assert_eq!(actual.secret_hash, expected["secretHash"].as_str().unwrap());
    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        actual.recipient_id,
        expected["recipientId"].as_str().unwrap()
    );
    if actual.vendor_field.is_some() {
        assert_eq!(
            actual.vendor_field.unwrap(),
            expected["vendorField"].as_str().unwrap()
        );
    }
    assert_eq!(
        actual.expiration_type as u64,
        expected["expirationType"].as_u64().unwrap()
    );
    assert_eq!(
        actual.expiration_value as u64,
        expected["expirationValue"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, expected["timestamp"].borrow());
}

pub fn test_transaction_array(actual: Vec<Transaction>, expected: Value) {
    for (pos, trx) in actual.iter().enumerate() {
        assert_transaction_data(trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_vote_array(actual: Vec<Transaction>, expected: Value) {
    for (pos, vote_trx) in actual.iter().enumerate() {
        assert_vote_data(vote_trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_lock_array(actual: Vec<Lock>, expected: Value) {
    for (pos, lock) in actual.iter().enumerate() {
        assert_lock_data(lock.clone(), &expected["data"][pos]);
    }
}
