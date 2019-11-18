use serde_json::{from_str, Value};

use crate::utils::assert_helpers::{assert_transaction_data, assert_block_data};
use crate::utils::mockito_helpers::{mock_client, mock_http_request};

#[test]
fn test_blocks_all() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let mut client = mock_client();
        let response = client.blocks.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        for i in 1..=response.data.len() {
            let rest_block = response.data[i-1].clone();
            let deser_block = expected["data"][i-1].clone();
            assert_block_data(&rest_block, &deser_block);
        }
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

        for i in 0..=response.data.len() - 1 {
            let rest_trx = response.data[i].clone();
            let desered_trx = expected["data"][i].clone();
            assert_transaction_data(rest_trx, &desered_trx);
        }
    }
}

#[test]
fn test_search() {
    // TODO: missing fixture
    // let _mock = mock_http_request("blocks/search");
    // {
    //     let client = mock_client();
    //     let response = client.blocks.search(vec![("id", "dummy")]);
    //     //mock_assert_success(&_mock, "blocks/search", response);
    // }
}
