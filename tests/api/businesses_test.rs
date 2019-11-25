use crate::utils::asserts::bridgechain::test_bridgechain_array;
use crate::utils::asserts::business::test_business_array;
use crate::utils::asserts::meta::assert_meta;
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;

#[test]
fn test_businesses_all() {
    let (_mock, body) = mock_http_request("businesses");
    {
        let mut client = mock_client();
        let response = client.businesses.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_business_array(response.data, expected);
    }
}

#[test]
#[ignore]
fn test_businesses_params() {
    // TODO: implement test
}

#[test]
fn test_businesses_bridgechains() {
    let (_mock, body) = mock_http_request("businesses/dummy/bridgechains");
    {
        let mut client = mock_client();
        let business_address = "dummy";
        let response = client.businesses.bridgechains(business_address).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_bridgechain_array(response.data, expected);
    }
}

#[test]
#[ignore]
fn test_businesses_search() {
    // TODO: implement test
}

#[test]
#[ignore]
fn test_businesses_show() {
    // TODO: implement test
}
