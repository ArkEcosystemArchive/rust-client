use *;
use serde_json::{from_str};

use arkecosystem_client::api::two::models::{Asset, Transaction};

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("votes");
    {
        let client = mock_client_two();
        let actual = client.votes.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_vote(actual_data, expected_data);
    }
}

#[test]
fn test_all_params() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request_two("votes");
    {
        let client = mock_client_two();
        let params = [("limit", "20")].iter();
        let actual = client.votes.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_vote(actual_data, expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("votes/dummy");
    {
        let client = mock_client_two();
        let actual = client.votes.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_vote(actual.data, expected["data"].clone());
    }
}

fn assert_vote(actual: Transaction, expected: Value) {
    assert_eq!(
        actual.id,
        expected["id"].as_str().unwrap()
    );
    assert_eq!(
        actual.block_id,
        expected["blockId"].as_str().unwrap()
    );
    assert_eq!(
        actual.transaction_type as u64,
        expected["type"].as_u64().unwrap()
    );
    assert_eq!(
        actual.amount,
        expected["amount"].as_u64().unwrap()
    );
    assert_eq!(
        actual.fee,
        expected["fee"].as_u64().unwrap()
    );
    assert_eq!(
        actual.sender,
        expected["sender"].as_str().unwrap()
    );
    assert_eq!(
        actual.recipient,
        expected["recipient"].as_str().unwrap()
    );
    assert_eq!(
        actual.signature,
        expected["signature"].as_str().unwrap()
    );
    match actual.asset {
        Asset::Votes(votes) => {
            assert_eq!(
                votes[0],
                expected["asset"]["votes"][0].as_str().unwrap()
            );
        },
        _ => panic!("Asset without votes")
    };
    assert_eq!(
        actual.confirmations,
        expected["confirmations"].as_u64().unwrap()
    );
    assert_eq!(
        actual.timestamp.epoch,
        expected["timestamp"]["epoch"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.timestamp.unix,
        expected["timestamp"]["unix"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.timestamp.human,
        expected["timestamp"]["human"].as_str().unwrap()
    );
}
