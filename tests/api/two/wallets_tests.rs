use test_helper::{mock_http_request_two, mock_client_two, mock_assert_success_two};

#[test]
fn test_all() {
    let _mock = mock_http_request_two("wallets");
    {
        let client = mock_client_two();
        let response = client.wallets.all(Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "wallets", response);
    }
}

#[test]
fn test_show() {
    let _mock = mock_http_request_two("wallets/dummy");
    {
        let client = mock_client_two();
        let response = client.wallets.show("dummy".to_owned());

        mock_assert_success_two(&_mock, "wallets/dummy", response);
    }
}

#[test]
fn test_transactions() {
    let _mock = mock_http_request_two("wallets/dummy/transactions");
    {
        let client = mock_client_two();
        let response = client.wallets.transactions("dummy".to_owned(), Vec::<(String, String)>::new());

        mock_assert_success_two(&_mock, "wallets/dummy/transactions", response);
    }
}

#[test]
fn test_sent_transactions() {
    let _mock = mock_http_request_two("wallets/dummy/transactions/sent");
    {
        let client = mock_client_two();
        let response = client.wallets.sent_transactions("dummy".to_owned(), Vec::<(String, String)>::new());

        mock_assert_success_two(&_mock, "wallets/dummy/transactions/sent", response);
    }
}

#[test]
fn test_received_transactions() {
    let _mock = mock_http_request_two("wallets/dummy/transactions/received");
    {
        let client = mock_client_two();
        let response = client.wallets.received_transactions("dummy".to_owned(), Vec::<(String, String)>::new());

        mock_assert_success_two(&_mock, "wallets/dummy/transactions/received", response);
    }
}

#[test]
fn test_votes() {
    let _mock = mock_http_request_two("wallets/dummy/votes");
    {
        let client = mock_client_two();
        let response = client.wallets.votes("dummy".to_owned());

        mock_assert_success_two(&_mock, "wallets/dummy/votes", response);
    }
}

#[test]
fn test_search() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("wallets/search");
    // {
    //     let client = mock_client_two();
    //     let response = client.wallets.search(Vec::<(String, String)>::new());
    //
    //     mock_assert_success_two(&_mock, "wallets/search", response);
    // }
}

#[test]
fn test_top() {
    let _mock = mock_http_request_two("wallets/top");
    {
        let client = mock_client_two();
        let response = client.wallets.top(Vec::<(String, String)>::new());

        mock_assert_success_two(&_mock, "wallets/top", response);
    }
}
