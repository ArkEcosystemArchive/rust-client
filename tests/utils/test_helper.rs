use arkecosystem_client::api::one::One;
use arkecosystem_client::api::two::Two;
use mockito::{mock, Mock, Matcher};
use serde_json::{Value};
use failure;

use std::fs::File;
use std::io::prelude::*;

const MOCK_HOST: &'static str = "http://127.0.0.1:1234/api/";

pub fn mock_http_request_one(endpoint: &str) -> Mock {
    let url = Matcher::Regex(endpoint.to_owned());
    println!("{:?}", url);
    mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&json!({"success": true}).to_string())
        .create()
}

pub fn mock_http_request_two(endpoint: &str) -> Mock {
    let url = Matcher::Regex(endpoint.to_owned());

    let fixture_name = endpoint.replace("-", "_") + ".json";
    let mut file = File::open(format!("tests/fixtures/two/{}", fixture_name)).unwrap();
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    println!("{:?}", url);
    mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create()
}

pub fn mock_client_one() -> One {
    One::new(&MOCK_HOST.to_owned())
}

pub fn mock_client_two() -> Two {
    Two::new(&MOCK_HOST.to_owned())
}

pub fn mock_assert_success(mock: &Mock, response: Result<Value, failure::Error>) {
    mock.assert();
    assert!(response.is_ok());

    let value = response.unwrap();
    assert!(value["success"] == true);
}
