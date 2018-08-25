extern crate arkecosystem_client;

use test_helper::{mock_http_request_one, mock_client_one, mock_assert_success};

#[test]
fn test_all() {
    let _mock = mock_http_request_one("delegates");
    {
        let client = mock_client_one();
        let response = client.delegates.all(vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_show() {
    let _mock = mock_http_request_one("delegates/get");
    {
        let client = mock_client_one();
        let response = client.delegates.show(vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_count() {
    let _mock = mock_http_request_one("delegates/count");
    {
        let client = mock_client_one();
        let response = client.delegates.count();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_search() {
    let _mock = mock_http_request_one("delegates/search");
    {
        let client = mock_client_one();
        let response = client.delegates.search("dummy".to_owned(), vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_voters() {
    let _mock = mock_http_request_one("delegates/voters");
    {
        let client = mock_client_one();
        let response = client.delegates.voters("dummy".to_owned(), vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_fee() {
    let _mock = mock_http_request_one("delegates/fee");
    {
        let client = mock_client_one();
        let response = client.delegates.fee();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_forged_by_account() {
    let _mock = mock_http_request_one("delegates/forging/getForgedByAccount");
    {
        let client = mock_client_one();
        let response = client.delegates.forged_by_account("dummy".to_owned());
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_next_forgers() {
    let _mock = mock_http_request_one("delegates/getNextForgers");
    {
        let client = mock_client_one();
        let response = client.delegates.next_forgers();
        mock_assert_success(&_mock, response);
    }
}

#[test]
fn test_forging_status() {
    let _mock = mock_http_request_one("delegates/forging/status");
    {
        let client = mock_client_one();
        let response = client.delegates.forging_status("dummy".to_owned(), vec![("", "")]);
        mock_assert_success(&_mock, response);
    }
}
