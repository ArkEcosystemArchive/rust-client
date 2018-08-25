extern crate arkecosystem_client;

use test_helper::{mock_http_request_one, mock_client_one, mock_assert_success};

#[test]
fn test_all() {
    let _mock = mock_http_request_one("peers");
    {
        let client = mock_client_one();
        let response = client.peers.all(vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_status() {
    let _mock = mock_http_request_one("peers/get");
    {
        let client = mock_client_one();
        let response = client.peers.status("ip".to_owned(), "port".to_owned());
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_version() {
    let _mock = mock_http_request_one("peers/version");
    {
        let client = mock_client_one();
        let response = client.peers.version();
        mock_assert_success(&_mock, response);
    }
}
