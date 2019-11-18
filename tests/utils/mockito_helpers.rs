use std::fs::File;
use std::io::prelude::*;

use mockito::{Matcher, mock, Mock};

use arkecosystem_client::Connection;

const MOCK_HOST: &str = "http://127.0.0.1:1234/api/";

pub fn mock_http_request(endpoint: &str) -> (Mock, String) {
    let url = Matcher::Regex(endpoint.to_owned());
    let response_body = read_fixture(&endpoint);

    let mock = mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create();


    //TODO: check if we can remove section
    // Delegates: replace integers in response body which are deserialized from serde as floats
    // to correctly match.
    /*if endpoint.contains("delegate") {
        response_body =
            response_body.replace("\"productivity\": 100\n", "\"productivity\": 100.0\n");
    }*/

    (mock, response_body.to_owned())
}

pub fn mock_post_request(endpoint: &str) -> (Mock, String) {
    let url = Matcher::Regex(endpoint.to_owned());
    let response_body = read_fixture(&endpoint);

    let mock = mock("POST", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create();

    (mock, response_body.to_owned())
}

pub fn mock_client() -> Connection {
    Connection::new(&MOCK_HOST)
}

fn read_fixture(endpoint: &str) -> String {
    let fixture_name = endpoint.replace("/", "-") + ".json";
    let mut file = File::open(format!("tests/fixtures/{}", fixture_name)).unwrap();
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    response_body
}
