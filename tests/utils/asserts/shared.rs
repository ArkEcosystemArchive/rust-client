use arkecosystem_client::api::models::timestamp::Timestamp;
use serde_json::Value;

pub fn assert_timestamp_data(actual: &Timestamp, expected: &Value) {
    assert_eq!(actual.epoch, expected["epoch"].as_u64().unwrap() as u32);
    assert_eq!(actual.unix, expected["unix"].as_u64().unwrap() as u32);
    assert_eq!(actual.human, expected["human"].as_str().unwrap());
}
