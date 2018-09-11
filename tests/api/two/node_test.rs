use *;
use serde_json::{from_str, to_string_pretty, Value};

#[test]
fn test_status() {
    let (_mock, body) = mock_http_request_two("node/status");
    {
        let client = mock_client_two();
        let response = client.node.status().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
    }
}

#[test]
fn test_syncing() {
    let (_mock, body) = mock_http_request_two("node/syncing");
    {
        let client = mock_client_two();
        let response = client.node.syncing().unwrap();
        let actual = to_string_pretty(&response).unwrap();
        assert_eq!(actual, body);
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

        // oder of ports is not the same, just compare it manually
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
        assert_eq!(
            actual.data.constants.fees.transfer,
            expected["data"]["constants"]["fees"]["transfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.second_signature,
            expected["data"]["constants"]["fees"]["secondSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.delegate_registration,
            expected["data"]["constants"]["fees"]["delegateRegistration"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.vote,
            expected["data"]["constants"]["fees"]["vote"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.multi_signature,
            expected["data"]["constants"]["fees"]["multiSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.ipfs,
            expected["data"]["constants"]["fees"]["ipfs"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.timelock_transfer,
            expected["data"]["constants"]["fees"]["timelockTransfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.multi_payment,
            expected["data"]["constants"]["fees"]["multiPayment"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.fees.delegate_resignation,
            expected["data"]["constants"]["fees"]["delegateResignation"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.transfer,
            expected["data"]["constants"]["dynamicOffsets"]["transfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.second_signature,
            expected["data"]["constants"]["dynamicOffsets"]["secondSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.delegate_registration,
            expected["data"]["constants"]["dynamicOffsets"]["delegateRegistration"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.vote,
            expected["data"]["constants"]["dynamicOffsets"]["vote"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.multi_signature,
            expected["data"]["constants"]["dynamicOffsets"]["multiSignature"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.ipfs,
            expected["data"]["constants"]["dynamicOffsets"]["ipfs"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.timelock_transfer,
            expected["data"]["constants"]["dynamicOffsets"]["timelockTransfer"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.multi_payment,
            expected["data"]["constants"]["dynamicOffsets"]["multiPayment"]
                .as_u64()
                .unwrap()
        );
        assert_eq!(
            actual.data.constants.dynamic_offsets.delegate_resignation,
            expected["data"]["constants"]["dynamicOffsets"]["delegateResignation"]
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
    }
}
