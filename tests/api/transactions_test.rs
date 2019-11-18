use std::collections::HashMap;

use serde_json::from_str;
use serde_json::Value;

use crate::utils::assert_helpers::{assert_meta, assert_transaction_data};
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("transactions");
    {
        let mut client = mock_client();
        let actual = client.transactions.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_transaction_data(actual_data, &expected_data);
    }
}

#[test]
fn test_all_param() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request("transactions");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.transactions.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_transaction_data(actual_data, &expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("transactions/dummy");
    {
        let mut client = mock_client();
        let actual = client.transactions.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_data(actual.data, &expected["data"]);
    }
}

#[test]
fn test_all_unconfirmed() {
    // TODO fixture with data
    let (_mock, body) = mock_http_request("transactions/unconfirmed");
    {
        let mut client = mock_client();
        let actual = client.transactions.all_unconfirmed().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);
    }
}

#[test]
fn test_all_unconfirmed_params() {
    // TODO current fixture does not have unconfirmed transactions
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request("transactions/unconfirmed");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.transactions.all_unconfirmed_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);
    }
}

#[test]
#[ignore]
fn test_show_unconfirmed() {
    // TODO: missing fixture
    // let (_mock, body) = mock_http_request("transactions/unconfirmed/dummy");
    // {
    //     let client = mock_client();
    //     let response = client.transactions.show_unconfirmed("dummy".to_owned());
    //
    //     mock_assert_success(&_mock, "transactions/unconfirmed/dummy", response);
    // }
}

#[test]
fn test_search() {
    let (_mock, body) = mock_post_request("transactions/search");
    {
        let mut client = mock_client();
        let mut query = HashMap::new();
        query.insert("senderId", "dummy");

        let params = [("limit", "20")].iter();
        let actual = client.transactions.search(Some(query), params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_transaction_data(actual_data, &expected_data);
    }
}

#[test]
#[ignore]
fn test_types() {
    /*let (_mock, body) = mock_http_request("transactions/types");
    {
        let mut client = mock_client();
        let actual = client.transactions.types().unwrap();
         println!("{:?}", actual);

        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.transfer,
            expected["data"]["1"]["Transfer"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.second_signature,
            expected["data"]["1"]["SecondSignature"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_registration,
            expected["data"]["1"]["DelegateRegistration"]
                .as_u64()
                .unwrap() as u16
        );
        assert_eq!(
            actual.data.vote,
            expected["data"]["1"]["Vote"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.multi_signature,
            expected["data"]["1"]["MultiSignature"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.ipfs,
            expected["data"]["1"]["Ipfs"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.multi_payment,
            expected["data"]["1"]["MultiPayment"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_resignation,
            expected["data"]["1"]["HtlcLock"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_resignation,
            expected["data"]["1"]["HtlcClaim"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_resignation,
            expected["data"]["1"]["HtlcRefund"].as_u64().unwrap() as u16
        );
        assert_eq!(
            actual.data.delegate_resignation,
            expected["data"]["1"]["DelegateResignation"]
                .as_u64()
                .unwrap() as u16
        );
    }*/
}

#[test]
#[ignore]
fn test_create() {
    let (_mock, body) = mock_post_request("transactions");
    {
        let mut client = mock_client();
        let actual = client.transactions.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_data(actual.data, &expected["data"].clone());
    }
}

#[test]
fn test_fees() {
    let (_mock, body) = mock_http_request("transactions/fees");
    {
        let mut client = mock_client();
        let actual = client.transactions.fees().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.transfer,
            expected["data"]["transfer"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.second_signature,
            expected["data"]["secondSignature"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.delegate_registration,
            expected["data"]["delegateRegistration"].as_u64().unwrap()
        );
        assert_eq!(actual.data.vote, expected["data"]["vote"].as_u64().unwrap());
        assert_eq!(
            actual.data.multi_signature,
            expected["data"]["multiSignature"].as_u64().unwrap()
        );
        assert_eq!(actual.data.ipfs, expected["data"]["ipfs"].as_u64().unwrap());
        assert_eq!(
            actual.data.multi_payment,
            expected["data"]["multiPayment"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.delegate_resignation,
            expected["data"]["delegateResignation"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.htlc_lock,
            expected["data"]["htlcLock"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.htlc_claim,
            expected["data"]["htlcClaim"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.htlc_refund,
            expected["data"]["htlcRefund"].as_u64().unwrap()
        );
    }
}
