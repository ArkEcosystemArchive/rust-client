use *;
use serde_json::{from_str};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("transactions");
    {
        let client = mock_client_two();
        let actual = client.transactions.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_transaction_data(actual_data, expected_data);
    }
}

#[test]
fn test_all_param() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request_two("transactions");
    {
        let client = mock_client_two();
        let params = [("limit", "20")].iter();
        let actual = client.transactions.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_transaction_data(actual_data, expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("transactions/dummy");
    {
        let client = mock_client_two();
        let actual = client.transactions.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_data(actual.data, expected["data"].clone());
    }
}

#[test]
fn test_all_unconfirmed() {
    // TODO fixture with data
    let (_mock, body) = mock_http_request_two("transactions/unconfirmed");
    {
        let client = mock_client_two();
        let actual = client.transactions.all_unconfirmed().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);
    }
}

#[test]
fn test_all_unconfirmed_params() {
    // TODO current fixture does not have unconfirmed transactions
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request_two("transactions/unconfirmed");
    {
        let client = mock_client_two();
        let params = [("limit", "20")].iter();
        let actual = client.transactions.all_unconfirmed_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);
    }
}

#[test]
#[ignore]
fn test_show_unconfirmed() {
    // TODO: missing fixture
    // let (_mock, body) = mock_http_request_two("transactions/unconfirmed/dummy");
    // {
    //     let client = mock_client_two();
    //     let response = client.transactions.show_unconfirmed("dummy".to_owned());
    //
    //     mock_assert_success_two(&_mock, "transactions/unconfirmed/dummy", response);
    // }
}

#[test]
fn test_search() {
    let (_mock, body) = mock_post_request("transactions/search");
    {
        let client = mock_client_two();
        let query = [("senderId", "dummy")].iter();
        let params = [("limit", "20")].iter();
        let actual = client.transactions.search(Some(query), params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_transaction_data(actual_data, expected_data);
    }
}

#[test]
fn test_types() {
    let (_mock, body) = mock_http_request_two("transactions/types");
    {
        let client = mock_client_two();
        let actual = client.transactions.types().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.transfer,
            expected["data"]["TRANSFER"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.second_signature,
            expected["data"]["SECOND_SIGNATURE"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_registration,
            expected["data"]["DELEGATE_REGISTRATION"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.vote,
            expected["data"]["VOTE"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.multi_signature,
            expected["data"]["MULTI_SIGNATURE"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.ipfs,
            expected["data"]["IPFS"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.timelock_transfer,
            expected["data"]["TIMELOCK_TRANSFER"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.multi_payment,
            expected["data"]["MULTI_PAYMENT"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_resignation,
            expected["data"]["DELEGATE_RESIGNATION"].as_u64().unwrap() as u16
        );
    }
}

#[test]
#[ignore]
fn test_create() {
    let (_mock, body) = mock_post_request("transactions");
    {
        let client = mock_client_two();
        let actual = client.transactions.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_data(actual.data, expected["data"].clone());
    }
}
