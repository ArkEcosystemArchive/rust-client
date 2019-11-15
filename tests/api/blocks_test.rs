use serde_json::to_string_pretty;
use serde_json::{from_str, Value};

use crate::common::{assert_transaction_data, mock_client, mock_http_request};

#[test]
fn test_blocks_all() {
    let (_mock, body) = mock_http_request("blocks");
    {
        let mut client = mock_client();
        let response = client.blocks.all().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("blocks/dummy");
    {
        let mut client = mock_client();
        let response = client.blocks.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        //TODO: add more tests, with type comparission. old actual/body are not relevant any more
        assert_eq!(
            response.data.payload.hash,
            expected["data"]["payload"]["hash"].as_str().unwrap()
        );
    }
}

#[test]
fn test_block_transactions() {
    let (_mock, body) = mock_http_request("blocks/dummy/transactions");
    {
        let mut client = mock_client();
        let response = client.blocks.transactions("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        for i in 0..=2 {
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
