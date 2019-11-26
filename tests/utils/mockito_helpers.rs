use arkecosystem_client::Connection;
use mockito::{mock, Matcher, Mock};
use std::fs::File;
use std::io::prelude::*;

const MOCK_HOST: &str = "http://127.0.0.1:1234/api/";

pub fn mock_http_request(endpoint: &str) -> (Mock, String) {
    let url = Matcher::Regex(endpoint.to_owned());
    let response_body = read_fixture(&endpoint, false);

    let mock = mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create();

    (mock, response_body)
}

pub fn mock_post_request(endpoint: &str) -> (Mock, String) {
    let url = Matcher::Regex(endpoint.to_owned());
    let response_body = read_fixture(&endpoint, true);

    let mock = mock("POST", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create();

    (mock, response_body)
}

pub fn mock_client() -> Connection {
    Connection::new(&MOCK_HOST)
}

fn read_fixture(endpoint: &str, post_request: bool) -> String {
    let fixture_name = if post_request {
        endpoint.replace("/", "-") + "-post.json"
    } else {
        endpoint.replace("/", "-") + ".json"
    };

    let mut file = File::open(format!("tests/fixtures/{}", fixture_name)).unwrap();
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    response_body
}
