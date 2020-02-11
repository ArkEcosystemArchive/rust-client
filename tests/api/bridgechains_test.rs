use crate::utils::asserts::bridgechain::{assert_bridgechain_data, test_bridgechain_array};
use crate::utils::asserts::meta::assert_meta;
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;
use std::collections::HashMap;

#[tokio::test]
async fn test_bridgechain_all() {
    let (_mock, body) = mock_http_request("bridgechains");
    {
        let mut client = mock_client();
        let response = client.bridgechains.all().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_bridgechain_array(response.data, expected);
    }
}

#[tokio::test]
async fn test_bridgechain_search() {
    let (_mock, body) = mock_post_request("bridgechains/search");
    {
        let mut client = mock_client();
        let mut payload = HashMap::new();
        payload.insert(
            "genesisHash",
            "82099a7396cb670d34b80f262571f5e82b7c7653ddedbc3ca42f51642f1cd516",
        );

        let params = [("limit", "20")].iter();
        let response = client.bridgechains.search(payload, params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_bridgechain_array(response.data, expected);
    }
}

#[tokio::test]
async fn test_bridgechain_show() {
    let (_mock, body) = mock_http_request("bridgechains/dummy");
    {
        let mut client = mock_client();
        let response = client.bridgechains.show("dummy").await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_bridgechain_data(&response.data, &expected["data"]);
    }
}
