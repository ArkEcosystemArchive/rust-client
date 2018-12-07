use *;
use serde_json::{from_str, Value};

use arkecosystem_client::api::two::models::Peer;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request_two("peers");
    {
        let client = mock_client_two();
        let actual = client.peers.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let meta = actual.meta.unwrap();
        assert_eq!(
            meta.count,
            expected["meta"]["count"].as_u64().unwrap() as u32
        );
        assert_eq!(
            meta.page_count,
            expected["meta"]["pageCount"].as_u64().unwrap() as u32
        );
        assert_eq!(
            meta.total_count,
            expected["meta"]["totalCount"].as_u64().unwrap() as u32
        );
        assert_eq!(
            meta.next.unwrap(),
            expected["meta"]["next"].as_str().unwrap()
        );
        assert_eq!(
            meta.previous.unwrap(),
            expected["meta"]["previous"].as_str().unwrap()
        );
        assert_eq!(
            meta.self_url,
            expected["meta"]["self"].as_str().unwrap()
        );
        assert_eq!(
            meta.first,
            expected["meta"]["first"].as_str().unwrap()
        );
        assert_eq!(
            meta.last.unwrap(),
            expected["meta"]["last"].as_str().unwrap()
        );

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_peer_data(actual_data, expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request_two("peers/dummy");
    {
        let client = mock_client_two();
        let actual = client.peers.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_peer_data(actual.data, expected["data"].clone());
    }
}

fn assert_peer_data(actual: Peer, expected: Value) {
    assert_eq!(
        actual.ip,
        expected["ip"].as_str().unwrap()
    );
    assert_eq!(
        actual.port,
        expected["port"].as_u64().unwrap() as u16
    );
    assert_eq!(
        actual.version,
        expected["version"].as_str().unwrap()
    );
    assert_eq!(
        actual.height,
        expected["height"].as_u64().unwrap()
    );
    assert_eq!(
        actual.status,
        expected["status"].as_u64().unwrap() as u16
    );
    assert_eq!(
        actual.os,
        expected["os"].as_str().unwrap()
    );
    assert_eq!(
        actual.latency,
        expected["latency"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.hashid,
        expected["hashid"].as_str().unwrap()
    );
}
