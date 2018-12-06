use *;
use serde_json::{from_str, Value};

#[test]
fn test_status() {
    let (_mock, body) = mock_http_request_two("node/status");
    {
        let client = mock_client_two();
        let response = client.node.status();
        let actual = response.unwrap();
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
        let client = mock_client_two();
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
        let client = mock_client_two();
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
        assert_eq!(
            dynamic_fees.addon_bytes.transfer,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["transfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.second_signature,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["secondSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.delegate_registration,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["delegateRegistration"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.vote,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["vote"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.multi_signature,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["multiSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.ipfs,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["ipfs"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.timelock_transfer,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["timelockTransfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.multi_payment,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["multiPayment"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            dynamic_fees.addon_bytes.delegate_resignation,
            expected["data"]["constants"]["fees"]["dynamicFees"]["addonBytes"]["delegateResignation"]
                .as_u64()
                .unwrap()
        );

        assert_eq!(
            actual.data.constants.fees.static_fees.transfer,
            expected["data"]["constants"]["fees"]["staticFees"]["transfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.second_signature,
            expected["data"]["constants"]["fees"]["staticFees"]["secondSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.delegate_registration,
            expected["data"]["constants"]["fees"]["staticFees"]["delegateRegistration"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.vote,
            expected["data"]["constants"]["fees"]["staticFees"]["vote"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.multi_signature,
            expected["data"]["constants"]["fees"]["staticFees"]["multiSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.ipfs,
            expected["data"]["constants"]["fees"]["staticFees"]["ipfs"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.timelock_transfer,
            expected["data"]["constants"]["fees"]["staticFees"]["timelockTransfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.multi_payment,
            expected["data"]["constants"]["fees"]["staticFees"]["multiPayment"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.static_fees.delegate_resignation,
            expected["data"]["constants"]["fees"]["staticFees"]["delegateResignation"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[0].transaction_type as u8,
            expected["data"]["feeStatistics"][0]["type"]
                .as_u64()
                .unwrap() as u8
        );
        assert_eq!(
            actual.data.fee_statistics[0].fees.min_fee,
            expected["data"]["feeStatistics"][0]["fees"]["minFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[0].fees.max_fee,
            expected["data"]["feeStatistics"][0]["fees"]["maxFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[0].fees.avg_fee,
            expected["data"]["feeStatistics"][0]["fees"]["avgFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[1].transaction_type as u8,
            expected["data"]["feeStatistics"][1]["type"]
                .as_u64()
                .unwrap() as u8
        );
        assert_eq!(
            actual.data.fee_statistics[1].fees.min_fee,
            expected["data"]["feeStatistics"][1]["fees"]["minFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[1].fees.max_fee,
            expected["data"]["feeStatistics"][1]["fees"]["maxFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[1].fees.avg_fee,
            expected["data"]["feeStatistics"][1]["fees"]["avgFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[2].transaction_type as u8,
            expected["data"]["feeStatistics"][2]["type"]
                .as_u64()
                .unwrap() as u8
        );
        assert_eq!(
            actual.data.fee_statistics[2].fees.min_fee,
            expected["data"]["feeStatistics"][2]["fees"]["minFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[2].fees.max_fee,
            expected["data"]["feeStatistics"][2]["fees"]["maxFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[2].fees.avg_fee,
            expected["data"]["feeStatistics"][2]["fees"]["avgFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[3].transaction_type as u8,
            expected["data"]["feeStatistics"][3]["type"]
                .as_u64()
                .unwrap() as u8
        );
        assert_eq!(
            actual.data.fee_statistics[3].fees.min_fee,
            expected["data"]["feeStatistics"][3]["fees"]["minFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[3].fees.max_fee,
            expected["data"]["feeStatistics"][3]["fees"]["maxFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[3].fees.avg_fee,
            expected["data"]["feeStatistics"][3]["fees"]["avgFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[4].transaction_type as u8,
            expected["data"]["feeStatistics"][4]["type"]
                .as_u64()
                .unwrap() as u8
        );
        assert_eq!(
            actual.data.fee_statistics[4].fees.min_fee,
            expected["data"]["feeStatistics"][4]["fees"]["minFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[4].fees.max_fee,
            expected["data"]["feeStatistics"][4]["fees"]["maxFee"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.fee_statistics[4].fees.avg_fee,
            expected["data"]["feeStatistics"][4]["fees"]["avgFee"]
                .as_u64()
                .unwrap()
        );
    }
}
