use test_helper::{mock_assert_success_one, mock_client_one, mock_http_request_one};

#[test]
fn test_all() {
    let _mock = mock_http_request_one("transactions");
    {
        let client = mock_client_one();
        let response = client.transactions.all(vec![("", "")]);
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_status() {
    let _mock = mock_http_request_one("transactions/get");
    {
        let client = mock_client_one();
        let response = client.transactions.show("ip".to_owned());
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_all_unconfirmed() {
    let _mock = mock_http_request_one("transactions/unconfirmed");
    {
        let client = mock_client_one();
        let response = client.transactions.all_unconfirmed(vec![("", "")]);
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_show_unconfirmed() {
    let _mock = mock_http_request_one("transactions/unconfirmed/get");
    {
        let client = mock_client_one();
        let response = client.transactions.show_unconfirmed("dummy".to_owned());
        mock_assert_success_one(&_mock, response);
    }
}
