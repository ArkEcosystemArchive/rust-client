use arkecosystem_client::api::models::transaction::{
    TransactionTypesCore, TransactionTypesMagistrate,
};
use serde_json::Value;

pub fn assert_transaction_types_core(core: TransactionTypesCore, expected: &Value) {
    assert_eq!(core.transfer, expected["Transfer"].as_u64().unwrap() as u16);
    assert_eq!(
        core.second_signature,
        expected["SecondSignature"].as_u64().unwrap() as u16
    );
    assert_eq!(
        core.delegate_registration,
        expected["DelegateRegistration"].as_u64().unwrap() as u16
    );
    assert_eq!(core.vote, expected["Vote"].as_u64().unwrap() as u16);
    assert_eq!(
        core.multi_signature,
        expected["MultiSignature"].as_u64().unwrap() as u16
    );
    assert_eq!(core.ipfs, expected["Ipfs"].as_u64().unwrap() as u16);
    assert_eq!(
        core.multi_payment,
        expected["MultiPayment"].as_u64().unwrap() as u16
    );
    assert_eq!(
        core.htlc_lock,
        expected["HtlcLock"].as_u64().unwrap() as u16
    );
    assert_eq!(
        core.htlc_claim,
        expected["HtlcClaim"].as_u64().unwrap() as u16
    );
    assert_eq!(
        core.htlc_refund,
        expected["HtlcRefund"].as_u64().unwrap() as u16
    );
    assert_eq!(
        core.delegate_resignation,
        expected["DelegateResignation"].as_u64().unwrap() as u16
    );
}

pub fn assert_transaction_types_magistrate(
    magistrate: TransactionTypesMagistrate,
    expected: &Value,
) {
    assert_eq!(
        magistrate.business_registration,
        expected["BusinessRegistration"].as_u64().unwrap() as u16
    );
    assert_eq!(
        magistrate.business_resignation,
        expected["BusinessResignation"].as_u64().unwrap() as u16
    );
    assert_eq!(
        magistrate.business_update,
        expected["BusinessUpdate"].as_u64().unwrap() as u16
    );
    assert_eq!(
        magistrate.bridgechain_registration,
        expected["BridgechainRegistration"].as_u64().unwrap() as u16
    );
    assert_eq!(
        magistrate.bridgechain_resignation,
        expected["BridgechainResignation"].as_u64().unwrap() as u16
    );
    assert_eq!(
        magistrate.bridgechain_update,
        expected["BridgechainUpdate"].as_u64().unwrap() as u16
    );
}
