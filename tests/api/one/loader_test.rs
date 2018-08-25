extern crate arkecosystem_client;

use test_helper::{mock_http_request_one, mock_client_one, mock_assert_success};

#[test]
fn test_status() {
    let _mock = mock_http_request_one("loader/status");
    {
        let client = mock_client_one();
        let response = client.loader.status();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_sync() {
    let _mock = mock_http_request_one("loader/status/sync");
    {
        let client = mock_client_one();
        let response = client.loader.sync();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_autoconfigure() {
    let _mock = mock_http_request_one("loader/autoconfigure");
    {
        let client = mock_client_one();
        let response = client.loader.autoconfigure();
        mock_assert_success(&_mock, response);
    }
}
