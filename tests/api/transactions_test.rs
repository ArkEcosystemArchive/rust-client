use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::transaction::{
    assert_transaction_data, assert_transaction_post_data, test_transaction_array,
};
use crate::utils::asserts::transaction_fees::{
    assert_transaction_core_fees, assert_transaction_magistrate_fees,
};
use crate::utils::asserts::transaction_types::{
    assert_transaction_types_core, assert_transaction_types_magistrate,
};
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};
use serde_json::from_str;
use serde_json::Value;
use std::borrow::Borrow;
use std::collections::HashMap;

#[tokio::test]
async fn test_all() {
    let (_mock, body) = mock_http_request("api/transactions");
    {
        let mut client = mock_client();
        let actual = client.transactions.all().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_all_param() {
    let (_mock, body) = mock_http_request("api/transactions");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.transactions.all_params(params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_show() {
    let (_mock, body) = mock_http_request("api/transactions/dummy");
    {
        let mut client = mock_client();
        let actual = client.transactions.show("dummy").await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_data(actual.data, &expected["data"]);
    }
}

#[tokio::test]
async fn test_all_unconfirmed() {
    let (_mock, body) = mock_http_request("api/transactions/unconfirmed");
    {
        let mut client = mock_client();
        let actual = client.transactions.all_unconfirmed().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_all_unconfirmed_params() {
    let (_mock, body) = mock_http_request("api/transactions/unconfirmed");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client
            .transactions
            .all_unconfirmed_params(params)
            .await
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_show_unconfirmed() {
    let (_mock, body) = mock_http_request("api/transactions/unconfirmed/dummy");
    {
        let mut client = mock_client();
        let actual = client.transactions.show_unconfirmed("dummy").await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_data(actual.data, &expected["data"]);
    }
}

#[tokio::test]
async fn test_search() {
    let (_mock, body) = mock_post_request("api/transactions/search");
    {
        let mut client = mock_client();
        let mut query = HashMap::new();
        query.insert("senderId", "dummy");

        let params = [("limit", "20")].iter();
        let actual = client.transactions.search(query, params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_transaction_types() {
    let (_mock, body) = mock_http_request("api/transactions/types");
    {
        let mut client = mock_client();
        let actual = client.transactions.types().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_types_core(actual.data.core, &expected["data"]["1"]);

        if let Some(magistrate) = actual.data.magistrate {
            assert_transaction_types_magistrate(magistrate, &expected["data"]["2"]);
        }
    }
}

#[tokio::test]
async fn test_create() {
    let (_mock, body) = mock_post_request("api/transactions");
    {
        let mut client = mock_client();
        let actual = client
            .transactions
            .create(Vec::<&str>::new())
            .await
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_post_data(actual.data, &expected["data"]);

        if let Some(errors) = actual.errors {
            let error = errors
                .get("3d3821a1e9271cd661f37e6cf1a2612e084d7cdc50a7b012c2bfff1413367b03")
                .unwrap();
            assert_eq!(error[0].error_type, "ERR_APPLY");
            assert_eq!(
                error[0].message,
                "Failed to apply transaction, because it votes for a resigned delegate."
            );
        }
    }
}

#[tokio::test]
async fn test_transaction_fees() {
    let (_mock, body) = mock_http_request("api/transactions/fees");
    {
        let mut client = mock_client();
        let actual = client.transactions.fees().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_transaction_core_fees(actual.data.core, &expected["data"]["1"]);

        if let Some(magistrate_fees) = actual.data.magistrate {
            assert_transaction_magistrate_fees(magistrate_fees, &expected["data"]["2"]);
        }
    }
}
