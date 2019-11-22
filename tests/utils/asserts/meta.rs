use arkecosystem_client::api::models::shared::Meta;
use serde_json::Value;

pub fn assert_meta(actual: Meta, expected: &Value) {
    if actual.count.is_some() {
        assert_eq!(
            actual.count.unwrap(),
            expected["count"].as_u64().unwrap() as u32
        );
    }
    if actual.page_count.is_some() {
        assert_eq!(
            actual.page_count.unwrap(),
            expected["pageCount"].as_u64().unwrap() as u32
        );
    }
    if actual.total_count.is_some() {
        assert_eq!(
            actual.total_count.unwrap(),
            expected["totalCount"].as_u64().unwrap() as u32
        );
    }
    if actual.next.is_some() {
        assert_eq!(actual.next.unwrap(), expected["next"].as_str().unwrap());
    }
    if actual.previous.is_some() {
        assert_eq!(
            actual.previous.unwrap(),
            expected["previous"].as_str().unwrap()
        );
    }
    if actual.self_url.is_some() {
        assert_eq!(actual.self_url.unwrap(), expected["self"].as_str().unwrap());
    }
    if actual.first.is_some() {
        assert_eq!(actual.first.unwrap(), expected["first"].as_str().unwrap());
    }
    if actual.last.is_some() {
        assert_eq!(actual.last.unwrap(), expected["last"].as_str().unwrap());
    }
    if actual.days.is_some() {
        assert_eq!(
            actual.days.unwrap(),
            expected["days"].as_u64().unwrap() as u32
        );
    }
}
