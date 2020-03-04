use serde_json::Value;
use std::str::FromStr;

use crate::utils::asserts::shared::assert_timestamp_data;
use arkecosystem_client::api::models::block::Block;

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
    if let Some(username) = &actual.generator.username {
        assert_eq!(
            username,
            expected["generator"]["username"].as_str().unwrap()
        );
    }
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

pub fn test_block_array(actual: Vec<Block>, expected: Value) {
    for (pos, block) in actual.iter().enumerate() {
        assert_block_data(block, &expected["data"][pos]);
    }
}
