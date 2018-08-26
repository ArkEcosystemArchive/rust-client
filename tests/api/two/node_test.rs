use test_helper::{mock_http_request_two, mock_client_two, mock_assert_success_two};

#[test]
fn test_status() {
    let _mock = mock_http_request_two("node/status");
    {
        let client = mock_client_two();
        let response = client.node.status();
        mock_assert_success_two(&_mock, "node/status", response);
    }
}

#[test]
fn test_syncing() {
    let _mock = mock_http_request_two("node/syncing");
    {
        let client = mock_client_two();
        let response = client.node.syncing();
        mock_assert_success_two(&_mock, "node/syncing", response);
    }
}

#[test]
fn test_configuration() {
    let _mock = mock_http_request_two("node/configuration");
    {
        let client = mock_client_two();
        let response = client.node.configuration();
        mock_assert_success_two(&_mock, "node/configuration", response);
    }
}
