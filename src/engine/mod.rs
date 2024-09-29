use client::{Client, ClientStore};
use transaction::{Transaction, TransactionStore};

use crate::{
    errors::ProcessingError,
    transaction_record::{TransactionRecord, TransactionRecordType},
};

mod client;
#[cfg(test)]
mod tests;
mod transaction;

#[derive(Default)]
pub struct TxEngine {
    clients_store: ClientStore,
    committed_txs: TransactionStore,
}

impl TxEngine {
    pub fn process_tx(&mut self, tx: TransactionRecord) -> Result<(), ProcessingError> {
        let client = self.clients_store.get_client_mut(tx.client);

        let tx_to_store = match tx.tx_type {
            TransactionRecordType::Deposit { amount } => {
                client.deposit(amount)?;
                Transaction::new(tx.tx, amount, tx.client)
            }
            TransactionRecordType::Withdrawal { amount } => {
                client.withdraw(amount)?;
                Transaction::new(tx.tx, amount, tx.client)
            }
            TransactionRecordType::Dispute
            | TransactionRecordType::Resolve
            | TransactionRecordType::Chargeback => {
                // In the below cases a clone of the transaction is created.
                // This is done to avoid mutating the original transaction
                // what would need to be reverted in case of any further errors.
                // When operation in client fails, error is returned before the transaction is inserted into the store.
                let referred_tx = self.committed_txs.get(&tx.tx)?;

                if referred_tx.client_id() != tx.client {
                    return Err(ProcessingError::ClientIdNotMatched);
                }

                match tx.tx_type {
                    TransactionRecordType::Dispute => {
                        let modified_tx = referred_tx.clone().disputed()?;
                        client.dispute(referred_tx.get_amount())?;
                        modified_tx
                    }
                    TransactionRecordType::Resolve => {
                        let modified_tx = referred_tx.clone().resolved()?;
                        client.resolve(referred_tx.get_amount())?;
                        modified_tx
                    }
                    TransactionRecordType::Chargeback => {
                        let modified_tx = referred_tx.clone().charged_back()?;
                        client.charge_back(referred_tx.get_amount())?;
                        modified_tx
                    }
                    _ => unreachable!(),
                }
            }
        };

        self.committed_txs.insert(tx_to_store);
        Ok(())
    }

    pub fn get_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients_store.get_clients()
    }
}
