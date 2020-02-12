use arkecosystem_client::api::models::fee::FeeStatistics;
use arkecosystem_client::api::models::transaction::TransactionFeesCore;
use serde_json::Value;
use std::str::FromStr;

pub fn assert_configuration_fees(actual: &TransactionFeesCore, expected: &Value) {
    assert_eq!(actual.transfer, expected["transfer"].as_u64().unwrap());
    assert_eq!(
        actual.second_signature,
        expected["secondSignature"].as_u64().unwrap()
    );
    assert_eq!(
        actual.delegate_registration,
        expected["delegateRegistration"].as_u64().unwrap()
    );
    assert_eq!(actual.vote, expected["vote"].as_u64().unwrap());
    assert_eq!(
        actual.multi_signature,
        expected["multiSignature"].as_u64().unwrap()
    );
    assert_eq!(actual.ipfs, expected["ipfs"].as_u64().unwrap());
    assert_eq!(
        actual.multi_payment,
        expected["multiPayment"].as_u64().unwrap()
    );
    assert_eq!(
        actual.delegate_resignation,
        expected["delegateResignation"].as_u64().unwrap()
    );
}

pub fn assert_node_fee_stats(actual: &FeeStatistics, expected: &Value) {
    assert_eq!(
        actual.avg,
        u64::from_str(expected["avg"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.min,
        u64::from_str(expected["min"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.max,
        u64::from_str(expected["max"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.sum,
        u64::from_str(expected["sum"].as_str().unwrap()).unwrap()
    );
}
