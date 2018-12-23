use serde_json::ser::to_string_pretty;
use *;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("wallets");
    {
        let client = mock_client();
        let response = client.wallets.all().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("wallets/dummy");
    {
        let client = mock_client();
        let response = client.wallets.show("dummy").unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions");
    {
        let client = mock_client();
        let response = client.wallets.transactions("dummy").unwrap();

        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_sent_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/sent");
    {
        let client = mock_client();
        let response = client.wallets.sent_transactions("dummy").unwrap();

        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_received_transactions() {
    let (_mock, body) = mock_http_request("wallets/dummy/transactions/received");
    {
        let client = mock_client();
        let response = client.wallets.received_transactions("dummy").unwrap();

        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_votes() {
    let (_mock, body) = mock_http_request("wallets/dummy/votes");
    {
        let client = mock_client();
        let response = client.wallets.votes("dummy").unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
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
    //     //mock_assert_success_two(&_mock, "wallets/search", response);
    // }
}

#[test]
fn test_top() {
    let (_mock, body) = mock_http_request("wallets/top");
    {
        let client = mock_client();
        let response = client.wallets.top().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}
