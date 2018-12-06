use *;
use serde_json::to_string_pretty;

#[test]
fn test_all_blocks() {
    let (_mock, body) = mock_http_request_two("blocks");
    {
        let client = mock_client_two();
        let response = client.blocks.all().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("blocks/dummy");
    {
        let client = mock_client_two();
        let response = client.blocks.show("dummy").unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_transactions() {
    let (_mock, body) = mock_http_request_two("blocks/dummy/transactions");
    {
        let client = mock_client_two();
        let response = client
            .blocks
            .transactions("dummy")
            .unwrap();

        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_search() {
    // TODO: missing fixture
    // let _mock = mock_http_request_two("blocks/search");
    // {
    //     let client = mock_client_two();
    //     let response = client.blocks.search(vec![("id", "dummy")]);
    //     //mock_assert_success_two(&_mock, "blocks/search", response);
    // }
}
