use mockito::server_address;
use serde_json::{from_str, Value};

use crate::utils::assert_helpers::assert_wallet_data;
use crate::utils::mockito_helpers::{mock_client, mock_http_request};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("wallets");
    {
        let mut client = mock_client();
        let response = client.wallets.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        for i in 1..=response.data.len() {
            let rest_wallet = response.data[i - 1].clone();
            let deser_wallet = expected["data"][i - 1].clone();

            assert_wallet_data(rest_wallet.clone(), &deser_wallet);
        }
    }
}

#[test]
#[ignore]
fn test_show() {
    let (_mock, body) = mock_http_request("wallets/dummy");
    {
        let mut client = mock_client();
        let response = client.wallets.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        // assert_wallet_data(response["data"].clone(), &expected);
    }
}

#[test]
#[ignore]
fn test_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions");
    {
        let mut client = mock_client();
        let response = client.wallets.transactions("dummy").unwrap();

        // let actual = to_string_pretty(&response).unwrap();
        // assert_eq!(actual, body);
    }
}

#[test]
#[ignore]
fn test_sent_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/sent");
    {
        let mut client = mock_client();
        let response = client.wallets.sent_transactions("dummy").unwrap();

        // let actual = to_string_pretty(&response).unwrap();
        // assert_eq!(actual, body);
    }
}

#[test]
#[ignore]
fn test_received_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/received");
    {
        let mut client = mock_client();
        let response = client.wallets.received_transactions("dummy").unwrap();

        // let actual = to_string_pretty(&response).unwrap();
        // assert_eq!(actual, body);
    }
}

#[test]
#[ignore]
fn test_votes() {
    let (_mock, body) = mock_http_request("wallets/dummy/votes");
    {
        let mut client = mock_client();
        let response = client.wallets.votes("dummy").unwrap();
        // let actual = to_string_pretty(&response).unwrap();
        // assert_eq!(actual, body);
    }
}

#[test]
#[ignore]
fn test_search() {
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
#[ignore]
fn test_top() {
    let (_mock, body) = mock_http_request("wallets/top");
    {
        let mut client = mock_client();
        let response = client.wallets.top().unwrap();
        // let actual = to_string_pretty(&response).unwrap();
        // assert_eq!(actual, body);
    }
}
