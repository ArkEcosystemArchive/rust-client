use crate::*;
use serde::Serialize;
use serde_json::ser::to_string_pretty;
use serde_json::Value;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("wallets");
    {
        let client = mock_client();
        client.wallets.all().unwrap();
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("wallets/dummy");
    {
        let client = mock_client();
        client.wallets.show("dummy").unwrap();
    }
}

#[test]
fn test_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions");
    {
        let client = mock_client();
        client.wallets.transactions("dummy").unwrap();
    }
}

#[test]
fn test_sent_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/sent");
    {
        let client = mock_client();
        client.wallets.sent_transactions("dummy").unwrap();
    }
}

#[test]
fn test_received_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/received");
    {
        let client = mock_client();
        client.wallets.received_transactions("dummy").unwrap();
    }
}

#[test]
fn test_votes() {
    let (_mock, body) = mock_http_request("wallets/dummy/votes");
    {
        let client = mock_client();
        client.wallets.votes("dummy").unwrap();
    }
}

#[test]
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
fn test_top() {
    let (_mock, body) = mock_http_request("wallets/top");
    {
        let client = mock_client();
        client.wallets.top().unwrap();
    }
}
