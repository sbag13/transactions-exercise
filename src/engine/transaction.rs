use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use crate::{errors::TransactionError, ClientId, Float, TransactionId};

#[derive(Clone)]
enum TransactionState {
    Committed,
    Disputed,
    Resolved,
    ChargedBack,
}

/// Transaction type used for processing in the engine, contains additional information
/// This type represents input transactions that includes the amount (withdrawal, deposit).
/// Input transactions that refers to a previous transaction (dispute, resolve, chargeback)
/// are reflected in a Transaction state.
/// This structure is invariant to the actual transaction type (withdrawal or deposit),
/// as it only uses the amount and client id.
#[derive(Clone)]
pub struct Transaction {
    id: TransactionId,
    amount: Float,
    client: ClientId,
    state: TransactionState,
}

type TransactionResult<T> = Result<T, TransactionError>;

// Custom implementation used to avoid exposing transaction details
impl Debug for Transaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transaction").finish()
    }
}

impl Transaction {
    pub fn new(id: TransactionId, amount: Float, client: ClientId) -> Self {
        Self {
            id,
            amount,
            client,
            state: TransactionState::Committed,
        }
    }

    pub fn get_amount(&self) -> Float {
        self.amount
    }

    pub fn disputed(mut self) -> TransactionResult<Self> {
        match self.state {
            TransactionState::Committed => {
                self.state = TransactionState::Disputed;
                Ok(self)
            }
            _ => Err(TransactionError::CannotBeDisputed),
        }
    }

    pub fn resolved(mut self) -> TransactionResult<Self> {
        match self.state {
            TransactionState::Disputed => {
                self.state = TransactionState::Resolved;
                Ok(self)
            }
            _ => Err(TransactionError::NotUnderDispute),
        }
    }

    pub fn charged_back(mut self) -> TransactionResult<Self> {
        match self.state {
            TransactionState::Disputed => {
                self.state = TransactionState::ChargedBack;
                Ok(self)
            }
            _ => Err(TransactionError::NotUnderDispute),
        }
    }

    pub fn client_id(&self) -> ClientId {
        self.client
    }
}

#[derive(Default)]
pub struct TransactionStore {
    store: HashMap<TransactionId, Transaction>,
}

impl TransactionStore {
    pub fn get(&mut self, id: &TransactionId) -> Result<&Transaction, TransactionError> {
        self.store
            .get(id)
            .ok_or(TransactionError::ReferredTxNotFound)
    }

    /// Inserts or updates a transaction in the store
    pub fn insert(&mut self, tx: Transaction) {
        self.store.insert(tx.id, tx);
    }
}
