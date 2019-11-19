use std::collections::HashMap;

use serde_json::{from_str, Value};

use crate::utils::assert_helpers::{
    assert_delegate_data, assert_meta, test_block_array, test_delegate_array, test_wallet_array,
};
use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};
use std::borrow::Borrow;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("delegates");
    {
        let mut client = mock_client();
        let actual = client.delegates.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_delegate_array(actual.data, expected);
    }
}

#[test]
fn test_all_params() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request("delegates");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.delegates.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_delegate_array(actual.data, expected);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("delegates/dummy");
    {
        let mut client = mock_client();
        let actual = client.delegates.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_delegate_data(actual.data, &expected["data"]);
    }
}

#[test]
fn test_blocks() {
    let (_mock, body) = mock_http_request("delegates/dummy/blocks");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let actual = client.delegates.blocks(delegate_address).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_block_array(actual.data, expected);
    }
}

#[test]
fn test_blocks_params() {
    let (_mock, body) = mock_http_request("delegates/dummy/blocks");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let params = [("limit", "10")].iter();
        let actual = client
            .delegates
            .blocks_params(delegate_address, params)
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_block_array(actual.data, expected);
    }
}

#[test]
fn test_voters() {
    let (_mock, body) = mock_http_request("delegates/dummy/voters");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let actual = client.delegates.voters(delegate_address).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(actual.data, expected);
    }
}

#[test]
fn test_voters_params() {
    let (_mock, body) = mock_http_request("delegates/dummy/voters");
    {
        let mut client = mock_client();
        let delegate_address = "dummy";
        let params = [("limit", "4")].iter();
        let actual = client
            .delegates
            .voters_params(delegate_address, params)
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_wallet_array(actual.data, expected);
    }
}

#[test]
fn test_search() {
    let (_mock, body) = mock_post_request("delegates/search");
    {
        let mut client = mock_client();
        let mut payload = HashMap::new();
        payload.insert("username", "dummy");

        let params = [("limit", "20")].iter();
        let actual = client.delegates.search(Some(payload), params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_delegate_array(actual.data, expected);
    }
}
