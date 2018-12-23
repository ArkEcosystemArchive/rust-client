extern crate arkecosystem_client;
extern crate failure;
extern crate mockito;
extern crate serde_json;
#[macro_use]
extern crate assert_float_eq;

mod api;
mod connection;

use mockito::{mock, Matcher, Mock};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

use arkecosystem_client::api::models::{Block, Meta, Timestamp, Transaction, Wallet};
use arkecosystem_client::Connection;

const MOCK_HOST: &str = "http://127.0.0.1:1234/api/";

pub fn mock_http_request(endpoint: &str) -> (Mock, String) {
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
    let mut file = File::open(format!("tests/fixtures/two/{}", fixture_name)).unwrap();
    let mut response_body = String::new();
    file.read_to_string(&mut response_body).unwrap();

    response_body
}

fn assert_meta(actual: Meta, expected: &Value) {
    assert_eq!(actual.count, expected["count"].as_u64().unwrap() as u32);
    assert_eq!(
        actual.page_count,
        expected["pageCount"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.total_count,
        expected["totalCount"].as_u64().unwrap() as u32
    );
    if actual.next.is_some() {
        assert_eq!(actual.next.unwrap(), expected["next"].as_str().unwrap());
    }
    if actual.previous.is_some() {
        assert_eq!(
            actual.previous.unwrap(),
            expected["previous"].as_str().unwrap()
        );
    }
    assert_eq!(actual.self_url, expected["self"].as_str().unwrap());
    assert_eq!(actual.first, expected["first"].as_str().unwrap());
    if actual.last.is_some() {
        assert_eq!(actual.last.unwrap(), expected["last"].as_str().unwrap());
    }
}

fn assert_timestamp_data(actual: &Timestamp, expected: &Value) {
    assert_eq!(actual.epoch, expected["epoch"].as_u64().unwrap() as u32);
    assert_eq!(actual.unix, expected["unix"].as_u64().unwrap() as u32);
    assert_eq!(actual.human, expected["human"].as_str().unwrap());
}

fn assert_block(actual: &Block, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    assert_eq!(actual.version, expected["version"].as_u64().unwrap() as u8);
    assert_eq!(actual.height, expected["height"].as_u64().unwrap());
    assert_eq!(actual.previous, expected["previous"].as_str().unwrap());
    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());
    assert_eq!(
        actual.forged.reward,
        expected["forged"]["reward"].as_u64().unwrap()
    );
    assert_eq!(
        actual.forged.fee,
        expected["forged"]["fee"].as_u64().unwrap()
    );
    assert_eq!(
        actual.forged.total,
        expected["forged"]["total"].as_u64().unwrap()
    );
    assert_eq!(
        actual.forged.amount,
        expected["forged"]["amount"].as_u64().unwrap()
    );
    assert_eq!(
        actual.payload.hash,
        expected["payload"]["hash"].as_str().unwrap()
    );
    assert_eq!(
        actual.payload.length,
        expected["payload"]["length"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.generator.username,
        expected["generator"]["username"].as_str().unwrap()
    );
    assert_eq!(
        actual.generator.address,
        expected["generator"]["address"].as_str().unwrap()
    );
    assert_eq!(
        actual.generator.public_key,
        expected["generator"]["publicKey"].as_str().unwrap()
    );
    assert_eq!(
        actual.transactions,
        expected["transactions"].as_u64().unwrap() as u32
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
}

fn assert_transaction_data(actual: Transaction, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    assert_eq!(actual.block_id, expected["blockId"].as_str().unwrap());
    if let Some(version) = actual.version {
        assert_eq!(version, expected["version"].as_u64().unwrap() as u16);
    }
    assert_eq!(
        actual.transaction_type as u64,
        expected["type"].as_u64().unwrap()
    );
    assert_eq!(actual.amount, expected["amount"].as_u64().unwrap());
    assert_eq!(actual.fee, expected["fee"].as_u64().unwrap());
    assert_eq!(actual.sender, expected["sender"].as_str().unwrap());
    if let Some(recipient) = actual.recipient {
        assert_eq!(recipient, expected["recipient"].as_str().unwrap());
    }
    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());
    if let Some(sign_signature) = actual.sign_signature {
        assert_eq!(sign_signature, expected["signSignature"].as_str().unwrap());
    }
    if let Some(vendor_field) = actual.vendor_field {
        assert_eq!(vendor_field, expected["vendorField"].as_str().unwrap());
    }

    // NOTE: asset should be tested on each transaction type

    assert_eq!(
        actual.confirmations,
        expected["confirmations"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
}

fn assert_wallet_data(actual: Wallet, expected: &Value) {
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    if let Some(public_key) = actual.public_key {
        assert_eq!(public_key, expected["publicKey"].as_str().unwrap());
    }
    if let Some(username) = actual.username {
        assert_eq!(username, expected["username"].as_str().unwrap());
    }
    if let Some(second_public_key) = actual.second_public_key {
        assert_eq!(
            second_public_key,
            expected["secondPublicKey"].as_str().unwrap()
        );
    }
    assert_eq!(actual.balance, expected["balance"].as_u64().unwrap());
    assert_eq!(
        actual.is_delegate,
        expected["isDelegate"].as_bool().unwrap()
    );
}
