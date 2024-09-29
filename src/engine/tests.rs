use crate::errors::TransactionError;

use super::*;

use utils::*;

#[test]
fn test_deposit() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.1111, 1).unwrap();

    let client = engine.get_clients().next().unwrap();
    assert_eq!(client.available(), 100.1111);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.total(), 100.1111);
}

#[test]
fn test_withdrawal_more_than_available() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.0, 1).unwrap();

    assert_eq!(
        withdrawal(&mut engine, 1, 200.0, 2).unwrap_err(),
        ProcessingError::InsufficientFunds
    );
}

#[test]
fn test_client_id_mismatch() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.0, 1).unwrap();

    assert_eq!(
        dispute(&mut engine, 2, 1).unwrap_err(),
        ProcessingError::ClientIdNotMatched
    );
}

#[test]
fn test_disputes_transaction_not_found() {
    let mut engine = TxEngine::default();

    assert_eq!(
        dispute(&mut engine, 1, 1).unwrap_err(),
        ProcessingError::InvalidTransaction(TransactionError::ReferredTxNotFound)
    );
}

#[test]
fn test_disputes_transaction_already_disputed() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.0, 1).unwrap();
    dispute(&mut engine, 1, 1).unwrap();

    assert_eq!(
        dispute(&mut engine, 1, 1).unwrap_err(),
        ProcessingError::InvalidTransaction(TransactionError::CannotBeDisputed)
    );
}

#[test]
fn test_resolve_disputed_transaction() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.0, 1).unwrap();

    dispute(&mut engine, 1, 1).unwrap();
    resolve(&mut engine, 1, 1).unwrap();

    let client = engine.get_clients().next().unwrap();
    assert_eq!(client.available(), 100.0);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.total(), 100.0);
}

#[test]
fn test_resolved_transaction_cannot_be_disputed_again() {
    let mut engine = TxEngine::default();
    deposit(&mut engine, 1, 100.0, 1).unwrap();
    dispute(&mut engine, 1, 1).unwrap();
    resolve(&mut engine, 1, 1).unwrap();

    assert_eq!(
        dispute(&mut engine, 1, 1).unwrap_err(),
        ProcessingError::InvalidTransaction(TransactionError::CannotBeDisputed)
    );
}

#[test]
fn test_chargeback_disputed_transaction() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.0, 1).unwrap();
    dispute(&mut engine, 1, 1).unwrap();
    chargeback(&mut engine, 1, 1).unwrap();

    let client = engine.get_clients().next().unwrap();
    assert_eq!(client.available(), 0.0);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.total(), 0.0);
    assert!(client.is_locked());
}

#[test]
fn test_withdrawal_after_chargeback() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 200.0, 1).unwrap();
    dispute(&mut engine, 1, 1).unwrap();
    chargeback(&mut engine, 1, 1).unwrap();

    assert_eq!(
        withdrawal(&mut engine, 1, 100.0, 2).unwrap_err(),
        ProcessingError::ClientLocked
    );
}

#[test]
fn test_negative_balance_after_dispute() {
    let mut engine = TxEngine::default();

    deposit(&mut engine, 1, 100.0, 1).unwrap();
    withdrawal(&mut engine, 1, 50.0, 2).unwrap();

    let client = engine.get_clients().next().unwrap();
    assert_eq!(client.available(), 50.0);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.total(), 50.0);

    dispute(&mut engine, 1, 1).unwrap();
    let client = engine.get_clients().next().unwrap();

    // is this correct ?
    assert_eq!(client.available(), -50.0);
    assert_eq!(client.held(), 100.0);
    assert_eq!(client.total(), 50.0);

    chargeback(&mut engine, 1, 1).unwrap();

    let client = engine.get_clients().next().unwrap();

    // is this correct ?
    assert_eq!(client.available(), -50.0);
    assert_eq!(client.held(), 0.0);
    assert_eq!(client.total(), -50.0);
}

mod utils {
    use crate::{ClientId, Float, TransactionId};

    use super::*;

    pub fn deposit(
        engine: &mut TxEngine,
        client: ClientId,
        amount: Float,
        tx: TransactionId,
    ) -> Result<(), ProcessingError> {
        engine.process_tx(TransactionRecord {
            tx_type: TransactionRecordType::Deposit { amount },
            client,
            tx,
        })
    }

    pub fn withdrawal(
        engine: &mut TxEngine,
        client: ClientId,
        amount: Float,
        tx: TransactionId,
    ) -> Result<(), ProcessingError> {
        engine.process_tx(TransactionRecord {
            tx_type: TransactionRecordType::Withdrawal { amount },
            client,
            tx,
        })
    }

    pub fn dispute(
        engine: &mut TxEngine,
        client: ClientId,
        tx: TransactionId,
    ) -> Result<(), ProcessingError> {
        engine.process_tx(TransactionRecord {
            tx_type: TransactionRecordType::Dispute,
            client,
            tx,
        })
    }

    pub fn resolve(
        engine: &mut TxEngine,
        client: ClientId,
        tx: TransactionId,
    ) -> Result<(), ProcessingError> {
        engine.process_tx(TransactionRecord {
            tx_type: TransactionRecordType::Resolve,
            client,
            tx,
        })
    }

    pub fn chargeback(
        engine: &mut TxEngine,
        client: ClientId,
        tx: TransactionId,
    ) -> Result<(), ProcessingError> {
        engine.process_tx(TransactionRecord {
            tx_type: TransactionRecordType::Chargeback,
            client,
            tx,
        })
    }
}
