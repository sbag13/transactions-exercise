#[derive(Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ProcessingError {
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Client is locked")]
    ClientLocked,
    #[error("Client ID does not match")]
    ClientIdNotMatched,
    #[error(transparent)]
    InvalidTransaction(#[from] TransactionError),
}

#[derive(Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TransactionError {
    #[error("Referred transaction not found")]
    ReferredTxNotFound,
    #[error("Referred transaction is already under dispute, resolved or charged back")]
    CannotBeDisputed,
    #[error("Referred transaction is not under dispute")]
    NotUnderDispute,
}
