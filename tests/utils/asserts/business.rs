use arkecosystem_client::api::models::business::Business;
use serde_json::Value;

pub fn assert_business_data(actual: &Business, expected: &Value) {
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    assert_eq!(actual.name, expected["name"].as_str().unwrap());
    assert_eq!(actual.public_key, expected["publicKey"].as_str().unwrap());
    assert_eq!(actual.website, expected["website"].as_str().unwrap());
    if let Some(vat) = &actual.vat {
        assert_eq!(vat, expected["vat"].as_str().unwrap());
    }
    if let Some(repository) = &actual.repository {
        assert_eq!(repository, expected["repository"].as_str().unwrap());
    }
}

pub fn test_business_array(actual: Vec<Business>, expected: Value) {
    for (pos, business) in actual.iter().enumerate() {
        assert_business_data(business, &expected["data"][pos]);
    }
}
