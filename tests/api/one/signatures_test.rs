use test_helper::{mock_assert_success_one, mock_client_one, mock_http_request_one};

#[test]
fn test_fee() {
    let _mock = mock_http_request_one("signatures/fee");
    {
        let client = mock_client_one();
        let response = client.signatures.fee();
        mock_assert_success_one(&_mock, response);
    }
}
