use *;
use serde_json::{from_str, Value};

use arkecosystem_client::api::models::{FeeSchema, FeeStatistics};

#[test]
fn test_status() {
    let (_mock, body) = mock_http_request_two("node/status");
    {
        let client = mock_client();
        let actual = client.node.status().unwrap();
        let expected: Value = from_str(&body).unwrap();

        assert_eq!(
            actual.data.synced,
            expected["data"]["synced"].as_bool().unwrap()
        );
        assert_eq!(
            actual.data.now,
            expected["data"]["now"].as_u64().unwrap()
        );
        assert_eq!(
            actual.data.blocks_count,
            expected["data"]["blocksCount"].as_i64().unwrap()
        );
    }
}

#[test]
fn test_syncing() {
    let (_mock, body) = mock_http_request_two("node/syncing");
    {
        let client = mock_client();
        let response = client.node.syncing();
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
        assert_eq!(
            actual.data.id,
            expected["data"]["id"].as_str().unwrap()
        );
    }
}

#[test]
fn test_configuration() {
    let (_mock, body) = mock_http_request_two("node/configuration");
    {
        let client = mock_client();
        let response = client.node.configuration();
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
        assert!(actual.data.ports.contains_key("@arkecosystem/core-graphql"));

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
        assert_eq!(
            actual.data.constants.fees.dynamic,
            expected["data"]["constants"]["fees"]["dynamic"]
                .as_bool()
                .unwrap()
        );

        let dynamic_fees = actual.data.constants.fees.dynamic_fees.unwrap();
        assert_eq!(
            dynamic_fees.min_fee_pool,
            expected["data"]["constants"]["fees"]["dynamicFees"]["minFeePool"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.min_fee_broadcast,
            expected["data"]["constants"]["fees"]["dynamicFees"]["minFeeBroadcast"]
                .as_u64()
                .unwrap()
        );

        assert_configuration_fees(
            &dynamic_fees.addon_bytes,
            &expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]
        );

        assert_configuration_fees(
            &actual.data.constants.fees.static_fees,
            &expected["data"]["constants"]["fees"]["staticFees"]
        );

        for i in 0..=4 {
            assert_fee_statistics(
                &actual.data.fee_statistics[i],
                &expected["data"]["feeStatistics"][i]
            );
        }
    }
}

fn assert_configuration_fees(actual: &FeeSchema, expected: &Value) {
    assert_eq!(
        actual.transfer,
        expected["transfer"].as_u64().unwrap()
    );
    assert_eq!(
        actual.second_signature,
        expected["secondSignature"].as_u64().unwrap()
    );
    assert_eq!(
        actual.delegate_registration,
        expected["delegateRegistration"].as_u64().unwrap()
    );
    assert_eq!(
        actual.vote,
        expected["vote"].as_u64().unwrap()
    );
    assert_eq!(
        actual.multi_signature,
        expected["multiSignature"].as_u64().unwrap()
    );
    assert_eq!(
        actual.ipfs,
        expected["ipfs"].as_u64().unwrap()
    );
    assert_eq!(
        actual.timelock_transfer,
        expected["timelockTransfer"].as_u64().unwrap()
    );
    assert_eq!(
        actual.multi_payment,
        expected["multiPayment"].as_u64().unwrap()
    );
    assert_eq!(
        actual.delegate_resignation,
        expected["delegateResignation"].as_u64().unwrap()
    );
}

fn assert_fee_statistics(actual: &FeeStatistics, expected: &Value) {
    assert_eq!(
        actual.transaction_type as u8,
        expected["type"].as_u64().unwrap() as u8
    );
    assert_eq!(
        actual.fees.min_fee,
        expected["fees"]["minFee"].as_u64().unwrap()
    );
    assert_eq!(
        actual.fees.max_fee,
        expected["fees"]["maxFee"].as_u64().unwrap()
    );
    assert_eq!(
        actual.fees.avg_fee,
        expected["fees"]["avgFee"].as_u64().unwrap()
    );
}
