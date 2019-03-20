use serde_json::from_str;
use std::collections::HashMap;
use *;

use arkecosystem_client::api::models::Delegate;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("delegates");
    {
        let client = mock_client();
        let actual = client.delegates.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_delegate_data(actual_data, &expected_data);
    }
}

#[test]
fn test_all_params() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request("delegates");
    {
        let client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.delegates.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_delegate_data(actual_data, &expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("delegates/dummy");
    {
        let client = mock_client();
        let actual = client.delegates.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_delegate_data(actual.data, &expected["data"]);
    }
}

#[test]
fn test_blocks() {
    let (_mock, body) = mock_http_request("delegates/dummy/blocks");
    {
        let client = mock_client();
        let delegate_address = "dummy";
        let actual = client.delegates.blocks(delegate_address).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_block(&actual_data, &expected_data);
    }
}

#[test]
fn test_blocks_params() {
    let (_mock, body) = mock_http_request("delegates/dummy/blocks");
    {
        let client = mock_client();
        let delegate_address = "dummy";
        let params = [("limit", "20")].iter();
        let actual = client
            .delegates
            .blocks_params(delegate_address, params)
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_block(&actual_data, &expected_data);
    }
}

#[test]
fn test_voters() {
    let (_mock, body) = mock_http_request("delegates/dummy/voters");
    {
        let client = mock_client();
        let delegate_address = "dummy";
        let actual = client.delegates.voters(delegate_address).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_wallet_data(actual_data, &expected_data);
    }
}

#[test]
fn test_voters_params() {
    let (_mock, body) = mock_http_request("delegates/dummy/voters");
    {
        let client = mock_client();
        let delegate_address = "dummy";
        let params = [("limit", "20")].iter();
        let actual = client
            .delegates
            .voters_params(delegate_address, params)
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_wallet_data(actual_data, &expected_data);
    }
}

#[test]
fn test_search() {
    let (_mock, body) = mock_post_request("delegates/search");
    {
        let client = mock_client();
        let mut payload = HashMap::new();
        payload.insert("username", "dummy");

        let params = [("limit", "20")].iter();
        let actual = client.delegates.search(Some(payload), params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_delegate_data(actual_data, &expected_data);
    }
}

fn assert_delegate_data(actual: Delegate, expected: &Value) {
    assert_eq!(actual.username, expected["username"].as_str().unwrap());
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    assert_eq!(actual.public_key, expected["publicKey"].as_str().unwrap());
    assert_eq!(actual.votes, expected["votes"].as_u64().unwrap());
    assert_eq!(actual.rank, expected["rank"].as_u64().unwrap() as u32);
    assert_eq!(
        actual.blocks.produced,
        expected["blocks"]["produced"].as_u64().unwrap()
    );
    assert_eq!(
        actual.blocks.missed,
        expected["blocks"]["missed"].as_u64().unwrap()
    );

    if actual.blocks.last.is_some() {
        let last = actual.blocks.last.unwrap().clone();
        assert_eq!(last.id, expected["blocks"]["last"]["id"].as_str().unwrap());
        assert_timestamp_data(
            &last.timestamp,
            &expected["blocks"]["last"]["timestamp"].clone(),
        );
    }

    assert_f64_near!(
        actual.production.approval,
        expected["production"]["approval"].as_f64().unwrap()
    );
    assert_f64_near!(
        actual.production.productivity,
        expected["production"]["productivity"].as_f64().unwrap()
    );
    assert_eq!(
        actual.forged.rewards,
        expected["forged"]["rewards"].as_u64().unwrap()
    );
    assert_eq!(
        actual.forged.fees,
        expected["forged"]["fees"].as_u64().unwrap()
    );
    assert_eq!(
        actual.forged.total,
        expected["forged"]["total"].as_u64().unwrap()
    );
}
