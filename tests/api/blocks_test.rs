use serde_json::{from_str, Value};

use crate::common::{assert_transaction_data, mock_client, mock_http_request, assert_block_data};

#[test]
fn test_blocks_all() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let mut client = mock_client();
        let response = client.blocks.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        for i in 0..=response.data.len() - 1 {
            let rest_block = response.data[i].clone();
            let deser_block = expected["data"][i].clone();
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
