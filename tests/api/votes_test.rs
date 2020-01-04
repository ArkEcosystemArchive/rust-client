use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::transaction::{assert_vote_data, test_vote_array};
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use serde_json::from_str;
use serde_json::Value;
use std::borrow::Borrow;

#[tokio::test]
async fn test_all() {
    let (_mock, body) = mock_http_request("votes");
    {
        let mut client = mock_client();
        let actual = client.votes.all().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_vote_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_all_params() {
    let (_mock, body) = mock_http_request("votes");
    {
        let mut client = mock_client();
        let params = [("limit", "20")].iter();
        let actual = client.votes.all_params(params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        test_vote_array(actual.data, expected);
    }
}

#[tokio::test]
async fn test_show() {
    let (_mock, body) = mock_http_request("votes/dummy");
    {
        let mut client = mock_client();
        let actual = client.votes.show("dummy").await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_vote_data(actual.data, &expected["data"]);
    }
}
