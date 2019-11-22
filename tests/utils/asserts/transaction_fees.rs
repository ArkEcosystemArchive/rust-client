use serde_json::Value;
use std::str::FromStr;

use arkecosystem_client::api::models::transaction::{
    TransactionFeesCore, TransactionFeesMagistrate,
};

pub fn assert_transaction_core_fees(core: TransactionFeesCore, expected: &Value) {
    assert_eq!(
        core.transfer,
        u64::from_str(expected["transfer"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        core.second_signature,
        u64::from_str(expected["secondSignature"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.delegate_registration,
        u64::from_str(expected["delegateResignation"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.vote,
        u64::from_str(expected["vote"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.multi_signature,
        u64::from_str(expected["multiSignature"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.ipfs,
        u64::from_str(expected["ipfs"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.multi_payment,
        u64::from_str(expected["multiPayment"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.delegate_resignation,
        u64::from_str(expected["delegateResignation"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.htlc_lock,
        u64::from_str(expected["htlcLock"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.htlc_claim,
        u64::from_str(expected["htlcClaim"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        core.htlc_refund,
        u64::from_str(expected["htlcRefund"].as_str().unwrap()).unwrap()
    );
}

pub fn assert_transaction_magistrate_fees(magistrate: TransactionFeesMagistrate, expected: &Value) {
    assert_eq!(
        magistrate.business_registration,
        u64::from_str(expected["businessRegistration"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        magistrate.business_resignation,
        u64::from_str(expected["businessResignation"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        magistrate.business_update,
        u64::from_str(expected["businessUpdate"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        magistrate.bridgechain_registration,
        u64::from_str(expected["bridgechainRegistration"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        magistrate.bridgechain_resignation,
        u64::from_str(expected["bridgechainResignation"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        magistrate.bridgechain_update,
        u64::from_str(expected["bridgechainUpdate"].as_str().unwrap()).unwrap()
    );
}
