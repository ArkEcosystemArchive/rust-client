use arkecosystem_client::api::models::bridgechain::Bridgechain;
use serde_json::Value;

pub fn assert_bridgechain_data(actual: &Bridgechain, expected: &Value) {
    assert_eq!(actual.public_key, expected["publicKey"].as_str().unwrap());
    assert_eq!(actual.name, expected["name"].as_str().unwrap());
    assert_eq!(
        actual.genesis_hash,
        expected["genesisHash"].as_str().unwrap()
    );
    assert_eq!(
        actual.bridgechain_repository,
        expected["bridgechainRepository"].as_str().unwrap()
    );

    assert!(actual.ports.contains_key("@arkecosystem/core-api"));

    if let Some(resigned) = actual.is_resigned {
        assert_eq!(resigned, expected["isResigned"].as_bool().unwrap())
    }
}

pub fn test_bridgechain_array(actual: Vec<Bridgechain>, expected: Value) {
    for (pos, bridgechain) in actual.iter().enumerate() {
        assert_bridgechain_data(bridgechain, &expected["data"][pos]);
    }
}
