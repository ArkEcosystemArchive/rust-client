use serde_json::Value;
use std::str::FromStr;

use crate::utils::asserts::shared::assert_timestamp_data;

use arkecosystem_client::api::models::delegate::Delegate;

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
        let last = actual.blocks.last.unwrap();
        assert_eq!(last.id, expected["blocks"]["last"]["id"].as_str().unwrap());
        assert_eq!(
            last.height,
            expected["blocks"]["last"]["height"].as_u64().unwrap()
        );

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

pub fn test_delegate_array(actual: Vec<Delegate>, expected: Value) {
    for (pos, delegate) in actual.iter().enumerate() {
        assert_delegate_data(delegate.clone(), &expected["data"][pos]);
    }
}
