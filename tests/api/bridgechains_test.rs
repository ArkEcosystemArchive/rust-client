use crate::utils::asserts::bridgechain::test_bridgechain_array;
use crate::utils::asserts::meta::assert_meta;
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;

#[test]
fn test_bridgechain_all() {
    let (_mock, body) = mock_http_request("bridgechains");
    {
        let mut client = mock_client();
        let response = client.bridgechains.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_bridgechain_array(response.data, expected);
    }
}

#[test]
#[ignore]
fn test_bridgechain_params() {
    // TODO: implement test
}

#[test]
#[ignore]
fn test_bridgechain_search() {
    // TODO: implement test
}

#[test]
#[ignore]
fn test_bridgechain_show() {
    // TODO: implement test
}
