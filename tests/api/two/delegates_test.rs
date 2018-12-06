use *;
use serde_json::to_string_pretty;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("delegates");
    {
        let client = mock_client_two();
        let response = client
            .delegates
            .all()
            .unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("delegates/dummy");
    {
        let client = mock_client_two();
        let response = client.delegates.show("dummy").unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_blocks() {
    let (_mock, body) = mock_http_request_two("delegates/dummy/blocks");
    {
        let client = mock_client_two();
        let response = client
            .delegates
            .blocks("dummy")
            .unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_voters() {
    let (_mock, body) = mock_http_request_two("delegates/dummy/voters");
    {
        let client = mock_client_two();
        let response = client
            .delegates
            .voters("dummy")
            .unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}
