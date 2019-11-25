use crate::utils::asserts::block::{assert_block_data, test_block_array};
use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::transaction::test_transaction_array;
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;

#[test]
fn test_blocks_all() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let mut client = mock_client();
        let response = client.blocks.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_block_array(response.data, expected);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("blocks/dummy");
    {
        let mut client = mock_client();
        let response = client.blocks.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_block_data(&response.data, &expected["data"]);
    }
}

#[test]
fn test_block_transactions() {
    let (_mock, body) = mock_http_request("blocks/dummy/transactions");
    {
        let mut client = mock_client();
        let response = client.blocks.transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(response.meta.unwrap(), expected["meta"].borrow());

        test_transaction_array(response.data, expected);
    }
}

#[test]
#[ignore]
fn test_search() {
    // TODO: missing test
    // let _mock = mock_http_request("blocks/search");
    // {
    //     let client = mock_client();
    //     let response = client.blocks.search(vec![("id", "dummy")]);
    //     //mock_assert_success(&_mock, "blocks/search", response);
    // }
}
