use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::transaction::{assert_lock_data, test_lock_array};
use arkecosystem_client::api::models::lock::Lock;
use arkecosystem_client::api::models::shared::Response;
use arkecosystem_client::Connection;
use serde_json::{from_str, Value};
use std::borrow::Borrow;
use std::collections::HashMap;

use crate::utils::mockito_helpers::{mock_client, mock_http_request, mock_post_request};

#[test]
fn test_locks_all() {
    let (_mock, body) = mock_http_request("locks");
    {
        let mut client = mock_client();
        let actual = client.locks.all().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_lock_array(actual.data, expected);
    }
}

#[test]
fn test_locks_all_params() {
    let (_mock, body) = mock_http_request("locks");
    {
        let mut client = mock_client();
        let params = [(
            "secretHash",
            "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
        )]
        .iter();
        let actual = client.locks.all_params(params).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_lock_array(actual.data, expected);
    }
}

#[test]
fn test_locks_show() {
    let (_mock, body) = mock_http_request("locks/dummy");
    {
        let mut client: Connection = mock_client();
        let actual: Response<Lock> = client.locks.show("dummy").unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_lock_data(actual.data, &expected["data"]);
    }
}

#[test]
fn test_locks_search() {
    let (_mock, body) = mock_post_request("locks/search");
    {
        let mut client = mock_client();
        let mut query = HashMap::new();
        query.insert(
            "secretHash",
            "6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b",
        );

        let actual = client
            .locks
            .search(query, Vec::<(String, String)>::new())
            .unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_lock_array(actual.data, expected);
    }
}

#[test]
fn test_locks_unlocked() {
    let (_mock, body) = mock_post_request("locks/unlocked");
    {
        let mut client = mock_client();
        let mut ids = Vec::new();
        ids.push("6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b");
        ids.push("16b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b");
        ids.push("26b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b");

        let actual = client.locks.unlocked(ids).unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_lock_array(actual.data, expected);
    }
}
