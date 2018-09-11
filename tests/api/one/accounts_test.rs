use *;

#[test]
fn test_balance() {
    let _mock = mock_http_request_one("accounts/getBalance");
    {
        let client = mock_client_one();
        let response = client.accounts.balance("dummy".to_owned());
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_public_key() {
    let _mock = mock_http_request_one("accounts/getPublicKey");
    {
        let client = mock_client_one();
        let response = client.accounts.public_key("dummy".to_owned());
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_delegate() {
    let _mock = mock_http_request_one("accounts/delegates");
    {
        let client = mock_client_one();
        let response = client.accounts.delegate("dummy".to_owned());
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_delegates_fee() {
    let _mock = mock_http_request_one("accounts/delegates/fee");
    {
        let client = mock_client_one();
        let response = client.accounts.delegates_fee();
        mock_assert_success_one(&_mock, response);
    }
}

#[test]
fn test_account() {
    let _mock = mock_http_request_one("accounts");
    {
        let client = mock_client_one();
        let response = client.accounts.account("dummy".to_owned());
        mock_assert_success_one(&_mock, response);
    }
}
