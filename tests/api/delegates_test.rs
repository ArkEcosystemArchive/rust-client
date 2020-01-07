use crate::utils::asserts::block::test_block_array;
use crate::utils::asserts::delegate::{assert_delegate_data, test_delegate_array};
use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::wallet::test_wallet_array;
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;
use std::collections::HashMap;

#[tokio::test]
async fn test_all() {
    let (_mock, body) = mock_http_request("api/delegates");
    {
        let mut client = mock_client();
        let actual = client.delegates.all().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_delegate_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_all_params() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request("api/delegates");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.delegates.all_params(params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_delegate_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_show() {
    let (_mock, body) = mock_http_request("api/delegates/dummy");
    {
        let mut client = mock_client();
        let actual = client.delegates.show("dummy").await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_delegate_data(actual.data, &expected["data"]);
    }
}

#[tokio::test]
async fn test_blocks() {
    let (_mock, body) = mock_http_request("api/delegates/dummy/blocks");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let actual = client.delegates.blocks(delegate_address).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_block_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_blocks_params() {
    let (_mock, body) = mock_http_request("api/delegates/dummy/blocks");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let params = [("limit", "10")].iter();
        let actual = client
            .delegates
            .blocks_params(delegate_address, params)
            .await
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_block_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_voters() {
    let (_mock, body) = mock_http_request("api/delegates/dummy/voters");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let actual = client.delegates.voters(delegate_address).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_voters_params() {
    let (_mock, body) = mock_http_request("api/delegates/dummy/voters");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let params = [("limit", "4")].iter();
        let actual = client
            .delegates
            .voters_params(delegate_address, params)
            .await
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_search() {
    let (_mock, body) = mock_post_request("api/delegates/search");
    {
        let mut client = mock_client();
        let mut payload = HashMap::new();
        payload.insert("username", "dummy");

        let params = [("limit", "20")].iter();
        let actual = client.delegates.search(payload, params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_delegate_array(actual.data, expected);
    }
}
