use serde_json::{from_str, Value};

use arkecosystem_client::api::models::peer::Peer;

use arkecosystem_client::Connection;
use arkecosystem_client::api::models::shared::Response;


use crate::utils::assert_helpers::{assert_peer_data, assert_meta};
use crate::utils::mockito_helpers::{mock_client, mock_http_request};


#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("peers");
    {
        let mut client = mock_client();
        let actual = client.peers.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_peer_data(&actual_data, &expected_data);
    }
}

#[test]
fn test_all_params() {
    // TODO use a different fixture to check that uses query strings
    let (_mock, body) = mock_http_request("peers");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.peers.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        let actual_meta = actual.meta.unwrap();
        let expected_meta = expected["meta"].clone();
        assert_meta(actual_meta, &expected_meta);

        let actual_data = actual.data[0].clone();
        let expected_data = expected["data"][0].clone();
        assert_peer_data(&actual_data, &expected_data);
    }
}

#[test]
fn test_show() {
    let (_mock, body) = mock_http_request("peers/dummy");
    {
        let mut client: Connection = mock_client();
        let actual: Response<Peer> = client.peers.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_peer_data(&actual.data, &expected["data"]);
    }
}

