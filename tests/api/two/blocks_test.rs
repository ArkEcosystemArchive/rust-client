use test_helper::{mock_http_request_two, mock_client_two, mock_assert_success_two};

#[test]
fn test_all() {
    let _mock = mock_http_request_two("blocks");
    {
        let client = mock_client_two();
        let response = client.blocks.all(Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "blocks", response);
    }
}

#[test]
fn test_show() {
    let _mock = mock_http_request_two("blocks/dummy");
    {
        let client = mock_client_two();
        let response = client.blocks.show("dummy".to_owned());

        mock_assert_success_two(&_mock, "blocks/dummy", response);
    }
}

#[test]
fn test_transactions() {
    let _mock = mock_http_request_two("blocks/dummy/transactions");
    {
        let client = mock_client_two();
        let response = client.blocks.transactions("dummy".to_owned(), Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "blocks/dummy/transactions", response);
    }
}

#[test]
fn test_search() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("blocks/search");
    // {
    //     let client = mock_client_two();
    //     let response = client.blocks.search(vec![("id", "dummy")]);
    //     mock_assert_success_two(&_mock, "blocks/search", response);
    // }
}
