use arkecosystem_client::api::models::peer::Peer;
use serde_json::Value;

pub fn assert_peer_data(actual: &Peer, expected: &Value) {
    assert_eq!(actual.ip, expected["ip"].as_str().unwrap());
    assert_eq!(actual.port, expected["port"].as_i64().unwrap() as i16);
    assert_eq!(actual.version, expected["version"].as_str().unwrap());
    assert_eq!(actual.height, expected["height"].as_u64().unwrap());
    assert_eq!(actual.latency, expected["latency"].as_i64().unwrap() as u32);

    assert_eq!(!actual.ports.is_empty(), true);
}

pub fn test_peer_array(actual: Vec<Peer>, expected: Value) {
    for (pos, peer) in actual.iter().enumerate() {
        assert_peer_data(peer, &expected["data"][pos]);
    }
}
