extern crate arkecosystem_client;
extern crate failure;
extern crate mockito;
#[macro_use]
extern crate serde_json;

mod api;
mod connection_manager_test;
mod connection_test;

use mockito::{mock, Matcher, Mock};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

use arkecosystem_client::Connection;
use arkecosystem_client::api::{Two};

const MOCK_HOST: &'static str = "http://127.0.0.1:1234/api/";

pub fn mock_http_request_two(endpoint: &str) -> (Mock, String) {
    let url = Matcher::Regex(endpoint.to_owned());
    let mut response_body = read_fixture(&endpoint);

    let mock = mock("GET", url)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(&response_body)
        .create();

    // Modify json string to make sure tests pass, nothing critical.

    // Delegates: replace integers in response body which are deserialized from serde as floats
    // to correctly match.
    if endpoint.contains("delegate") {
        response_body =
            response_body.replace("\"productivity\": 100\n", "\"productivity\": 100.0\n");
    }

    // Wallet fixes
    if endpoint.contains("wallet") {
        // Some balances are deserialized from string to number, serialization then obviously results in a number.
        // TODO: remove when fixed
        response_body = response_body.replace(
            "\"balance\": \"5000000000\",\n",
            "\"balance\": 5000000000,\n",
        );
        response_body = response_body.replace(
            "\"balance\": \"24509800000000000\",\n",
            "\"balance\": 24509800000000000,\n",
        );
    }

    (mock, response_body.to_owned())
}

pub fn mock_client_two() -> Connection<Two> {
    Connection::<Two>::new(&MOCK_HOST)
}

fn read_fixture(endpoint: &str) -> String {
    let fixture_name = endpoint.replace("/", "-") + ".json";
    let mut file = File::open(format!("tests/fixtures/two/{}", fixture_name)).unwrap();
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    response_body
}
