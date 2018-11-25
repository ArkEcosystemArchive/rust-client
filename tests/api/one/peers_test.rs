use *;

#[test]
fn test_all() {
    let _mock = mock_http_request_one("peers");
    {
        let client = mock_client_one();
        let response = client.peers.all(vec![("", "")]);
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_status() {
    let _mock = mock_http_request_one("peers/get");
    {
        let client = mock_client_one();
        let response = client.peers.status("ip", "port");
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_version() {
    let _mock = mock_http_request_one("peers/version");
    {
        let client = mock_client_one();
        let response = client.peers.version();
        mock_assert_success_one(&_mock, response);
    }
}
