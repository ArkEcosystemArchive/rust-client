use serde_json::{from_str, Value};

use arkecosystem_client::api::models::peer::Peer;

use arkecosystem_client::api::models::shared::Response;
use arkecosystem_client::Connection;

use crate::utils::assert_helpers::{assert_meta, assert_peer_data, test_peer_array};
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use std::borrow::Borrow;

#[test]
fn test_all() {
    let (_mock, body) = mock_http_request("peers");
    {
        let mut client = mock_client();
        let actual = client.peers.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_peer_array(actual.data, expected);
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

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_peer_array(actual.data, expected);
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
