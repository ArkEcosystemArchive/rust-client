use failure;
use mockito::{mock, Mock, Matcher};
use serde_json::{from_str, Value};
use std::fs::File;
use std::io::prelude::*;

use arkecosystem_client::api::one::One;
use arkecosystem_client::api::two::Two;
use arkecosystem_client::connection::Connection;

const MOCK_HOST: &'static str = "http://127.0.0.1:1234/api/";

pub fn mock_http_request_one(endpoint: &str) -> Mock {
    let url = Matcher::Regex(endpoint.to_owned());

    mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&json!({"success": true}).to_string())
        .create()
}

pub fn mock_http_request_two(endpoint: &str) -> Mock {
    let url = Matcher::Regex(endpoint.to_owned());
    let response_body = read_fixture(&endpoint);

    mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create()
}

pub fn mock_client_one() -> Connection<One> {
    Connection::<One>::one(&MOCK_HOST)
}

pub fn mock_client_two() -> Connection<Two> {
    Connection::<Two>::two(&MOCK_HOST)
}

pub fn mock_assert_success_one(mock: &Mock, response: Result<Value, failure::Error>) {
    mock.assert();
    assert!(response.is_ok());

    let value = response.unwrap();
    assert!(value["success"] == true);
}

pub fn mock_assert_success_two(mock: &Mock, endpoint: &str, response: Result<Value, failure::Error>) {
    mock.assert();
    assert!(response.is_ok());

    let value = response.unwrap();
    let fixture_value = get_fixture_as_value(&endpoint);

    assert_eq!(value, fixture_value);
}

fn get_fixture_as_value(endpoint: &str) -> Value {
    let response_body = read_fixture(&endpoint);
    from_str(&response_body).unwrap()
}

fn read_fixture(endpoint: &str) -> String {
    let fixture_name = endpoint.replace("/", "-") + ".json";
    let mut file = File::open(format!("tests/fixtures/two/{}", fixture_name)).unwrap();
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    response_body
}
