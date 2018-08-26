extern crate arkecosystem_client;

use test_helper::{mock_http_request_two, mock_client_two, mock_assert_success_two};

#[test]
fn test_all() {
    let _mock = mock_http_request_two("delegates");
    {
        let client = mock_client_two();
        let response = client.delegates.all(Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "delegates", response);
    }
}

#[test]
fn test_show() {
    let _mock = mock_http_request_two("delegates/dummy");
    {
        let client = mock_client_two();
        let response = client.delegates.show("dummy".to_owned());
        mock_assert_success_two(&_mock, "delegates/dummy", response);
    }
}

#[test]
fn test_blocks() {
    let _mock = mock_http_request_two("delegates/dummy/blocks");
    {
        let client = mock_client_two();
        let response = client.delegates.blocks("dummy".to_owned(), Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "delegates/dummy/blocks", response);
    }
}

#[test]
fn test_voters() {
    let _mock = mock_http_request_two("delegates/dummy/voters");
    {
        let client = mock_client_two();
        let response = client.delegates.voters("dummy".to_owned(), Vec::<(String, String)>::new());
        mock_assert_success_two(&_mock, "delegates/dummy/voters", response);
    }
}
