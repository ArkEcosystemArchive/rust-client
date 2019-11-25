use serde_json::Value;
use std::str::FromStr;

use crate::utils::asserts::business::assert_business_data;
use arkecosystem_client::api::models::wallet::Wallet;

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
    assert_eq!(
        actual.nonce,
        u64::from_str(expected["nonce"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.balance,
        u64::from_str(expected["balance"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        actual.is_delegate,
        expected["isDelegate"].as_bool().unwrap()
    );
    assert_eq!(
        actual.is_resigned,
        expected["isResigned"].as_bool().unwrap()
    );

    if let Some(vote) = actual.vote {
        assert_eq!(vote, expected["vote"].as_str().unwrap());
    }
    if let Some(business) = actual.business {
        assert_business_data(&business, &expected["business"]);
    }
}

pub fn test_wallet_array(actual: Vec<Wallet>, expected: Value) {
    for (pos, wallet) in actual.iter().enumerate() {
        assert_wallet_data(wallet.clone(), &expected["data"][pos]);
    }
}
