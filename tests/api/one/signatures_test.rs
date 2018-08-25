extern crate arkecosystem_client;

use test_helper::{mock_http_server, mock_client_one, mock_assert_success};

#[test]
fn test_fee() {
    let _mock = mock_http_server("signatures/fee");
    {
        let client = mock_client_one();
        let response = client.signatures.fee();
        mock_assert_success(&_mock, response);
    }
}
