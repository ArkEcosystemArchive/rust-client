use crate::utils::asserts::meta::assert_meta;
use crate::utils::asserts::node::{assert_configuration_fees, assert_node_fee_stats};
use crate::utils::mockito_helpers::{mock_client, mock_http_request};
use serde_json::{from_str, Value};
use std::borrow::Borrow;

#[tokio::test]
async fn test_node_status() {
    let (_mock, body) = mock_http_request("api/node/status");
    {
        let mut client = mock_client();
        let actual = client.node.status().await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.synced,
            expected["data"]["synced"].as_bool().unwrap()
        );
        assert_eq!(actual.data.now, expected["data"]["now"].as_u64().unwrap());
        assert_eq!(
            actual.data.blocks_count,
            expected["data"]["blocksCount"].as_i64().unwrap()
        );
        assert_eq!(
            actual.data.timestamp,
            expected["data"]["timestamp"].as_u64().unwrap()
        );
    }
}

#[tokio::test]
async fn test_node_syncing() {
    let (_mock, body) = mock_http_request("api/node/syncing");
    {
        let mut client = mock_client();
        let response = client.node.syncing().await;
        let actual = response.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.syncing,
            expected["data"]["syncing"].as_bool().unwrap()
        );
        assert_eq!(
            actual.data.blocks,
            expected["data"]["blocks"].as_i64().unwrap()
        );
        assert_eq!(
            actual.data.height,
            expected["data"]["height"].as_u64().unwrap()
        );
        assert_eq!(actual.data.id, expected["data"]["id"].as_str().unwrap());
    }
}

#[tokio::test]
async fn test_node_configuration() {
    let (_mock, body) = mock_http_request("api/node/configuration");
    {
        let mut client = mock_client();
        let response = client.node.configuration().await;
        let actual = response.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.nethash,
            expected["data"]["nethash"].as_str().unwrap()
        );
        assert_eq!(
            actual.data.token,
            expected["data"]["token"].as_str().unwrap()
        );
        assert_eq!(
            actual.data.symbol,
            expected["data"]["symbol"].as_str().unwrap()
        );
        assert_eq!(
            actual.data.explorer,
            expected["data"]["explorer"].as_str().unwrap()
        );
        assert_eq!(
            actual.data.version,
            expected["data"]["version"].as_u64().unwrap() as u32
        );

        assert!(actual.data.ports.contains_key("@arkecosystem/core-p2p"));
        assert!(actual.data.ports.contains_key("@arkecosystem/core-api"));

        assert_eq!(
            actual.data.constants.height,
            expected["data"]["constants"]["height"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.constants.reward,
            expected["data"]["constants"]["reward"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.constants.active_delegates,
            expected["data"]["constants"]["activeDelegates"]
                .as_u64()
                .unwrap() as u32
        );
        assert_eq!(
            actual.data.constants.blocktime,
            expected["data"]["constants"]["blocktime"].as_u64().unwrap() as u32
        );
        assert_eq!(
            actual.data.constants.block.version,
            expected["data"]["constants"]["block"]["version"]
                .as_u64()
                .unwrap() as u32
        );
        assert_eq!(
            actual.data.constants.block.max_transactions,
            expected["data"]["constants"]["block"]["maxTransactions"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.block.max_payload,
            expected["data"]["constants"]["block"]["maxPayload"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.epoch,
            expected["data"]["constants"]["epoch"].as_str().unwrap()
        );
        assert_configuration_fees(
            &actual.data.constants.fees.static_fees,
            &expected["data"]["constants"]["fees"]["staticFees"],
        );
    }
}

#[tokio::test]
async fn test_node_fees() {
    let (_mock, body) = mock_http_request("api/node/fees");
    {
        let mut client = mock_client();
        let params = [("days", "20")].iter();
        let actual = client.node.fees(params).await.unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_meta(actual.meta.unwrap(), expected["meta"].borrow());

        // TODO ref: introduce dynamic type where enum_name translates to field name
        assert_node_fee_stats(
            &actual.data.core.transfer.unwrap(),
            &expected["data"]["1"]["transfer"],
        );
        assert_node_fee_stats(
            &actual.data.core.second_signature.unwrap(),
            &expected["data"]["1"]["secondSignature"],
        );
        assert_node_fee_stats(
            &actual.data.core.delegate_registration.unwrap(),
            &expected["data"]["1"]["delegateRegistration"],
        );

        //        for (pos, fee_stat) in actual.data.iter().enumerate() {
        //            assert_node_fee_stats(fee_stat, &expected["data"][pos]);
        //        }
    }
}
