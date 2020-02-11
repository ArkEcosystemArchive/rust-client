use crate::utils::asserts::bridgechain::test_bridgechain_array;
use crate::utils::asserts::business::{assert_business_data, test_business_array};
use crate::utils::asserts::meta::assert_meta;
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;
use std::collections::HashMap;

#[tokio::test]
async fn test_businesses_all() {
    let (_mock, body) = mock_http_request("businesses");
    {
        let mut client = mock_client();
        let response = client.businesses.all().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_business_array(response.data, expected);
    }
}

#[tokio::test]
async fn test_businesses_show() {
    let (_mock, body) = mock_http_request("businesses/dummy");
    {
        let mut client = mock_client();
        let response = client.businesses.show("dummy").await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_business_data(&response.data, &expected["data"]);
    }
}

#[tokio::test]
async fn test_businesses_bridgechains() {
    let (_mock, body) = mock_http_request("businesses/dummy/bridgechains");
    {
        let mut client = mock_client();
        let business_address = "dummy";
        let response = client
            .businesses
            .bridgechains(business_address)
            .await
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_bridgechain_array(response.data, expected);
    }
}

#[tokio::test]
async fn test_businesses_params() {
    let (_mock, body) = mock_http_request("businesses");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let response = client.businesses.all_params(params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_business_array(response.data, expected);
    }
}

#[tokio::test]
async fn test_businesses_search() {
    let (_mock, body) = mock_post_request("businesses/search");
    {
        let mut client = mock_client();
        let mut payload = HashMap::new();
        payload.insert("address", "DFyLKkWs12QwDTi8BywQN5ssa5CMK3dr6d");

        let params = [("limit", "20")].iter();
        let response = client.businesses.search(payload, params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_business_array(response.data, expected);
    }
}
