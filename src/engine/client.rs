use std::collections::HashMap;

use serde::{Serialize, Serializer};

use crate::{errors::ProcessingError, ClientId, Float};

type ProcessingResult<T> = Result<T, ProcessingError>;

#[derive(Debug, Serialize)]
pub struct Client {
    #[serde(rename = "client")]
    id: ClientId,
    #[serde(serialize_with = "serialize_float")]
    available: Float,
    #[serde(serialize_with = "serialize_float")]
    held: Float,
    #[serde(serialize_with = "serialize_float")]
    total: Float,
    locked: bool,
}

fn serialize_float<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Format the float to 4 decimal places
    let formatted = format!("{:.4}", value);
    // Trim trailing zeros and the decimal point if it's not needed
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    serializer.serialize_str(trimmed)
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Self {
            id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }

    pub fn deposit(&mut self, amount: Float) -> ProcessingResult<()> {
        self.lockable_operation(|client| {
            client.available += amount;
            client.total += amount;
            Ok(())
        })
    }

    pub fn withdraw(&mut self, amount: Float) -> ProcessingResult<()> {
        self.lockable_operation(|client| {
            if client.available < amount {
                return Err(ProcessingError::InsufficientFunds);
            }
            client.available -= amount;
            client.total -= amount;
            Ok(())
        })
    }

    pub fn dispute(&mut self, amount: Float) -> ProcessingResult<()> {
        self.lockable_operation(|client| {
            client.available -= amount;
            client.held += amount;
            Ok(())
        })
    }

    pub fn resolve(&mut self, amount: Float) -> ProcessingResult<()> {
        self.lockable_operation(|client| {
            // held won't be less than 0, because it's only added by dispute
            client.held -= amount;
            client.available += amount;
            Ok(())
        })
    }

    pub fn charge_back(&mut self, amount: Float) -> ProcessingResult<()> {
        self.lockable_operation(|client| {
            client.held -= amount;
            client.total -= amount;
            client.locked = true;
            Ok(())
        })
    }

    /// Wraps the operation in a lock check. This is trivial case, but in case of changes
    /// it will be easier to maintain if the lock check is in one place
    fn lockable_operation<T>(
        &mut self,
        op: impl FnOnce(&mut Self) -> ProcessingResult<T>,
    ) -> ProcessingResult<T> {
        if !self.locked {
            op(self)
        } else {
            Err(ProcessingError::ClientLocked)
        }
    }

    #[cfg(test)]
    pub fn available(&self) -> Float {
        self.available
    }

    #[cfg(test)]
    pub fn held(&self) -> Float {
        self.held
    }

    #[cfg(test)]
    pub fn total(&self) -> Float {
        self.total
    }

    #[cfg(test)]
    pub fn is_locked(&self) -> bool {
        self.locked
    }
}

#[derive(Default)]
pub struct ClientStore {
    clients: HashMap<ClientId, Client>,
}

impl ClientStore {
    pub fn get_client_mut(&mut self, id: ClientId) -> &mut Client {
        self.clients.entry(id).or_insert_with(|| Client::new(id))
    }

    pub fn get_clients(&self) -> impl Iterator<Item = &Client> {
        self.clients.values()
    }
}
