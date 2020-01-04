use crate::utils::asserts::bridgechain::test_bridgechain_array;
use crate::utils::asserts::business::test_business_array;
use crate::utils::asserts::meta::assert_meta;
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;

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
#[ignore]
async fn test_businesses_params() {
    // TODO: implement test
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
#[ignore]
async fn test_businesses_search() {
    // TODO: implement test
}

#[tokio::test]
#[ignore]
async fn test_businesses_show() {
    // TODO: implement test
}
