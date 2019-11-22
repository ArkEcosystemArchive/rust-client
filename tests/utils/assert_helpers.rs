use serde_json::Value;
use std::borrow::Borrow;
use std::str::FromStr;

use arkecosystem_client::api::models::block::Block;
use arkecosystem_client::api::models::delegate::Delegate;
use arkecosystem_client::api::models::fee::FeeStatistics;
use arkecosystem_client::api::models::lock::Lock;
use arkecosystem_client::api::models::peer::Peer;
use arkecosystem_client::api::models::shared::Meta;
use arkecosystem_client::api::models::timestamp::Timestamp;
use arkecosystem_client::api::models::transaction::{
    Transaction, TransactionFeesCore, TransactionFeesMagistrate, TransactionPostResponse,
    TransactionTypesCore, TransactionTypesMagistrate,
};
use arkecosystem_client::api::models::wallet::Wallet;

pub fn assert_meta(actual: Meta, expected: &Value) {
    if actual.count.is_some() {
        assert_eq!(
            actual.count.unwrap(),
            expected["count"].as_u64().unwrap() as u32
        );
    }
    if actual.page_count.is_some() {
        assert_eq!(
            actual.page_count.unwrap(),
            expected["pageCount"].as_u64().unwrap() as u32
        );
    }
    if actual.total_count.is_some() {
        assert_eq!(
            actual.total_count.unwrap(),
            expected["totalCount"].as_u64().unwrap() as u32
        );
    }
    if actual.next.is_some() {
        assert_eq!(actual.next.unwrap(), expected["next"].as_str().unwrap());
    }
    if actual.previous.is_some() {
        assert_eq!(
            actual.previous.unwrap(),
            expected["previous"].as_str().unwrap()
        );
    }
    if actual.self_url.is_some() {
        assert_eq!(actual.self_url.unwrap(), expected["self"].as_str().unwrap());
    }
    if actual.first.is_some() {
        assert_eq!(actual.first.unwrap(), expected["first"].as_str().unwrap());
    }
    if actual.last.is_some() {
        assert_eq!(actual.last.unwrap(), expected["last"].as_str().unwrap());
    }
    if actual.days.is_some() {
        assert_eq!(
            actual.days.unwrap(),
            expected["days"].as_u64().unwrap() as u32
        );
    }
}

pub fn assert_timestamp_data(actual: &Timestamp, expected: &Value) {
    assert_eq!(actual.epoch, expected["epoch"].as_u64().unwrap() as u32);
    assert_eq!(actual.unix, expected["unix"].as_u64().unwrap() as u32);
    assert_eq!(actual.human, expected["human"].as_str().unwrap());
}

pub fn assert_block_data(actual: &Block, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    assert_eq!(actual.version, expected["version"].as_u64().unwrap() as u8);
    assert_eq!(actual.height, expected["height"].as_u64().unwrap());
    assert_eq!(actual.previous, expected["previous"].as_str().unwrap());
    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());

    assert_eq!(
        actual.forged.reward,
        u64::from_str(expected["forged"]["reward"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.fee,
        u64::from_str(expected["forged"]["fee"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.total,
        u64::from_str(expected["forged"]["total"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.amount,
        u64::from_str(expected["forged"]["amount"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.payload.hash,
        expected["payload"]["hash"].as_str().unwrap()
    );
    assert_eq!(
        actual.payload.length,
        expected["payload"]["length"].as_u64().unwrap() as u32
    );
    assert_eq!(
        actual.generator.username,
        expected["generator"]["username"].as_str().unwrap()
    );
    assert_eq!(
        actual.generator.address,
        expected["generator"]["address"].as_str().unwrap()
    );
    assert_eq!(
        actual.generator.public_key,
        expected["generator"]["publicKey"].as_str().unwrap()
    );
    assert_eq!(
        actual.transactions,
        expected["transactions"].as_u64().unwrap() as u32
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
}

pub fn assert_transaction_data(actual: Transaction, expected: &Value) {
    assert_eq!(actual.id, expected["id"].as_str().unwrap());
    if let Some(block_id) = actual.block_id {
        assert_eq!(block_id, expected["blockId"].as_str().unwrap());
    }
    if let Some(version) = actual.version {
        assert_eq!(version, expected["version"].as_u64().unwrap() as u16);
    }
    assert_eq!(
        actual.transaction_type as u64,
        expected["type"].as_u64().unwrap()
    );
    assert_eq!(
        actual.type_group as u64,
        expected["typeGroup"].as_u64().unwrap()
    );

    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        actual.fee,
        u64::from_str(expected["fee"].as_str().unwrap()).unwrap()
    );

    assert_eq!(actual.sender, expected["sender"].as_str().unwrap());
    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );

    if let Some(recipient) = actual.recipient {
        assert_eq!(recipient, expected["recipient"].as_str().unwrap());
    }
    assert_eq!(actual.signature, expected["signature"].as_str().unwrap());
    if let Some(sign_signature) = actual.sign_signature {
        assert_eq!(sign_signature, expected["signSignature"].as_str().unwrap());
    }
    if let Some(vendor_field) = actual.vendor_field {
        assert_eq!(vendor_field, expected["vendorField"].as_str().unwrap());
    }

    assert_eq!(
        actual.confirmations,
        expected["confirmations"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, &expected["timestamp"].clone());
    if let Some(nonce) = actual.nonce {
        assert_eq!(nonce, expected["nonce"].as_str().unwrap());
    }
}

pub fn assert_wallet_data(actual: Wallet, expected: &Value) {
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    if let Some(public_key) = actual.public_key {
        assert_eq!(public_key, expected["publicKey"].as_str().unwrap());
    }
    if let Some(username) = actual.username {
        assert_eq!(username, expected["username"].as_str().unwrap());
    }
    if let Some(second_public_key) = actual.second_public_key {
        assert_eq!(
            second_public_key,
            expected["secondPublicKey"].as_str().unwrap()
        );
    }
    assert_eq!(
        actual.nonce,
        u64::from_str(expected["nonce"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.balance,
        u64::from_str(expected["balance"].as_str().unwrap()).unwrap()
    );

    assert_eq!(
        actual.is_delegate,
        expected["isDelegate"].as_bool().unwrap()
    );
    assert_eq!(
        actual.is_resigned,
        expected["isResigned"].as_bool().unwrap()
    );
}

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

pub fn assert_transaction_post_data(actual: TransactionPostResponse, expected: &Value) {
    for (pos, value) in actual.accept.iter().enumerate() {
        assert_eq!(value, &expected["accept"][pos]);
    }
    for (pos, value) in actual.broadcast.iter().enumerate() {
        assert_eq!(value, &expected["broadcast"][pos]);
    }
    for (pos, value) in actual.excess.iter().enumerate() {
        assert_eq!(value, &expected["excess"][pos]);
    }
    for (pos, value) in actual.invalid.iter().enumerate() {
        assert_eq!(value, &expected["invalid"][pos]);
    }
}

pub fn assert_delegate_data(actual: Delegate, expected: &Value) {
    assert_eq!(actual.username, expected["username"].as_str().unwrap());
    assert_eq!(actual.address, expected["address"].as_str().unwrap());
    assert_eq!(actual.public_key, expected["publicKey"].as_str().unwrap());
    assert_eq!(actual.votes, expected["votes"].as_str().unwrap());
    assert_eq!(actual.rank, expected["rank"].as_u64().unwrap() as u32);
    assert_eq!(
        actual.blocks.produced,
        expected["blocks"]["produced"].as_u64().unwrap()
    );

    if actual.blocks.last.is_some() {
        let last = actual.blocks.last.unwrap();
        assert_eq!(last.id, expected["blocks"]["last"]["id"].as_str().unwrap());
        assert_eq!(
            last.height,
            expected["blocks"]["last"]["height"].as_u64().unwrap()
        );

        assert_timestamp_data(
            &last.timestamp,
            &expected["blocks"]["last"]["timestamp"].clone(),
        );
    }
    assert_eq!(
        actual.forged.rewards,
        u64::from_str(expected["forged"]["rewards"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.fees,
        u64::from_str(expected["forged"]["fees"].as_str().unwrap()).unwrap()
    );
    assert_eq!(
        actual.forged.total,
        u64::from_str(expected["forged"]["total"].as_str().unwrap()).unwrap()
    );
}

pub fn assert_peer_data(actual: &Peer, expected: &Value) {
    assert_eq!(actual.ip, expected["ip"].as_str().unwrap());
    assert_eq!(actual.port, expected["port"].as_i64().unwrap() as i16);
    assert_eq!(actual.version, expected["version"].as_str().unwrap());
    assert_eq!(actual.height, expected["height"].as_u64().unwrap());
    assert_eq!(actual.latency, expected["latency"].as_i64().unwrap() as u32);

    assert_eq!(!actual.ports.is_empty(), true);
}

pub fn assert_node_fee_stats(actual: &FeeStatistics, expected: &Value) {
    assert_eq!(
        actual.avg,
        u64::from_str(expected["avg"].as_str().unwrap()).unwrap()
    );
    //    assert_eq!(actual.max, expected["max"]);
    //    assert_eq!(actual.min, expected["min"]);
    //    assert_eq!(actual.sum, expected["sum"]);
}

pub fn assert_vote_data(actual: Transaction, expected: &Value) {
    assert_transaction_data(actual.clone(), &expected);

    //    match actual.asset {
    //        Asset::Votes(votes) => {
    //            assert_eq!(votes[0], expected["asset"]["votes"][0].as_str().unwrap());
    //        }
    //        _ => panic!("Asset without votes"),
    //    };
}

pub fn assert_lock_data(actual: Lock, expected: &Value) {
    assert_eq!(actual.lock_id, expected["lockId"].as_str().unwrap());
    assert_eq!(
        actual.amount,
        u64::from_str(expected["amount"].as_str().unwrap()).unwrap()
    );
    assert_eq!(actual.secret_hash, expected["secretHash"].as_str().unwrap());
    assert_eq!(
        actual.sender_public_key,
        expected["senderPublicKey"].as_str().unwrap()
    );
    assert_eq!(
        actual.recipient_id,
        expected["recipientId"].as_str().unwrap()
    );
    if actual.vendor_field.is_some() {
        assert_eq!(
            actual.vendor_field.unwrap(),
            expected["vendorField"].as_str().unwrap()
        );
    }
    assert_eq!(
        actual.expiration_type as u64,
        expected["expirationType"].as_u64().unwrap()
    );
    assert_eq!(
        actual.expiration_value as u64,
        expected["expirationValue"].as_u64().unwrap()
    );
    assert_timestamp_data(&actual.timestamp, expected["timestamp"].borrow());
}

pub fn test_transaction_array(actual: Vec<Transaction>, expected: Value) {
    for (pos, trx) in actual.iter().enumerate() {
        assert_transaction_data(trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_wallet_array(actual: Vec<Wallet>, expected: Value) {
    for (pos, wallet) in actual.iter().enumerate() {
        assert_wallet_data(wallet.clone(), &expected["data"][pos]);
    }
}

pub fn test_delegate_array(actual: Vec<Delegate>, expected: Value) {
    for (pos, delegate) in actual.iter().enumerate() {
        assert_delegate_data(delegate.clone(), &expected["data"][pos]);
    }
}

pub fn test_block_array(actual: Vec<Block>, expected: Value) {
    for (pos, block) in actual.iter().enumerate() {
        assert_block_data(block, &expected["data"][pos]);
    }
}

pub fn test_peer_array(actual: Vec<Peer>, expected: Value) {
    for (pos, peer) in actual.iter().enumerate() {
        assert_peer_data(peer, &expected["data"][pos]);
    }
}

pub fn test_vote_array(actual: Vec<Transaction>, expected: Value) {
    for (pos, vote_trx) in actual.iter().enumerate() {
        assert_vote_data(vote_trx.clone(), &expected["data"][pos]);
    }
}

pub fn test_lock_array(actual: Vec<Lock>, expected: Value) {
    for (pos, lock) in actual.iter().enumerate() {
        assert_lock_data(lock.clone(), &expected["data"][pos]);
    }
}
