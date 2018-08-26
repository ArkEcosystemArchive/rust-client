use test_helper::{mock_http_request_two, mock_client_two, mock_assert_success_two};

#[test]
fn test_all() {
    let _mock = mock_http_request_two("transactions");
    {
        let client = mock_client_two();
        let response = client.transactions.all(Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "transactions", response);
    }
}

#[test]
fn test_show() {
    let _mock = mock_http_request_two("transactions/dummy");
    {
        let client = mock_client_two();
        let response = client.transactions.show("dummy".to_owned());

        mock_assert_success_two(&_mock, "transactions/dummy", response);
    }
}

#[test]
fn test_show_unconfirmed() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("transactions/unconfirmed/dummy");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.show_unconfirmed("dummy".to_owned());
    //
    //     mock_assert_success_two(&_mock, "transactions/unconfirmed/dummy", response);
    // }
}

#[test]
fn test_all_unconfirmed() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("transactions/unconfirmed");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.all_unconfirmed(Vec::<(String, String)>::new());
    //     mock_assert_success_two(&_mock, "transactions/unconfirmed", response);
    // }
}

#[test]
fn test_search() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("transactions/search");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.search(vec![("id", "dummy")]);
    //     mock_assert_success_two(&_mock, "transactions/search", response);
    // }
}
