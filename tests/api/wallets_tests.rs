use serde_json::{from_str, Value};

use arkecosystem_client::api::models::transaction::Transaction;
use arkecosystem_client::api::models::wallet::Wallet;

use crate::utils::assert_helpers::{assert_transaction_data, assert_wallet_data};
use crate::utils::mockito_helpers::{mock_client, mock_http_request};

#[test]
fn test_wallets_all() {
    let (_mock, body) = mock_http_request("wallets");
    {
        let mut client = mock_client();
        let response = client.wallets.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        test_wallet_array(response.data.clone(), expected.clone());
    }
}

#[test]
fn test_wallet_show() {
    let (_mock, body) = mock_http_request("wallets/dummy");
    {
        let mut client = mock_client();
        let response = client.wallets.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_wallet_data(response.data.clone(), &expected["data"]);
    }
}

#[test]
fn test_wallet_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions");
    {
        let mut client = mock_client();
        let response = client.wallets.transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        test_transaction_array(response.data.clone(), expected.clone());
    }
}

#[test]
fn test_wallet_sent_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/sent");
    {
        let mut client = mock_client();
        let response = client.wallets.sent_transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        test_transaction_array(response.data.clone(), expected.clone());
    }
}

#[test]
fn test_wallet_received_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/received");
    {
        let mut client = mock_client();
        let response = client.wallets.received_transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        test_transaction_array(response.data.clone(), expected.clone());
    }
}

#[test]
fn test_votes() {
    let (_mock, body) = mock_http_request("wallets/dummy/votes");
    {
        let mut client = mock_client();
        let response = client.wallets.votes("dummy").unwrap();

        let expected: Value = from_str(&body).unwrap();

        test_transaction_array(response.data.clone(), expected.clone());
    }
}

#[test]
#[ignore]
fn test_wallet_search() {
    // TODO: missing fixture
    // let (_mock, body) = mock_http_request("wallets/search");
    // {
    //     let client = mock_client();
    //     let response = client.wallets.search(Vec::<(String, String)>::new()).unwrap();
    //
    //     //mock_assert_success(&_mock, "wallets/search", response);
    // }
}

#[test]
fn test_wallet_top() {
    let (_mock, body) = mock_http_request("wallets/top");
    {
        let mut client = mock_client();
        let response = client.wallets.top().unwrap();
        let expected: Value = from_str(&body).unwrap();

        test_wallet_array(response.data.clone(), expected.clone());
    }
}

fn test_transaction_array(actual: Vec<Transaction>, expected: Value) {
    for i in 1..=actual.len() {
        let rest_trx = &actual[i - 1];
        let deser_trx = &expected["data"][i - 1];

        assert_transaction_data(rest_trx.clone(), deser_trx);
    }
}

fn test_wallet_array(actual: Vec<Wallet>, expected: Value) {
    for i in 1..=actual.len() {
        let rest_wallet = actual[i - 1].clone();
        let deser_wallet = expected["data"][i - 1].clone();

        assert_wallet_data(rest_wallet.clone(), &deser_wallet);
    }
}
