use std::fmt::{self, Debug, Display};

use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::{ClientId, Float, TransactionId};

#[derive(Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub enum TransactionRecordType {
    Deposit { amount: Float },
    Withdrawal { amount: Float },
    Dispute,
    Resolve,
    Chargeback,
}

impl<'de> Deserialize<'de> for TransactionRecordType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Custom visitor to handle case-insensitive transaction type deserialization
        struct TransactionTypeVisitor;

        impl<'de> Visitor<'de> for TransactionTypeVisitor {
            type Value = TransactionRecordType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a transaction type object with a case-insensitive type field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut transaction_type: Option<String> = None;
                let mut amount: Option<Float> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "type" => {
                            if transaction_type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;
                            transaction_type = Some(value.to_lowercase()); // Case-insensitive
                        }
                        "amount" => {
                            if amount.is_some() {
                                return Err(de::Error::duplicate_field("amount"));
                            }
                            amount = Some(map.next_value()?);
                        }
                        _ => return Err(de::Error::unknown_field(&key, &["type", "amount"])),
                    }
                }

                let transaction_type =
                    transaction_type.ok_or_else(|| de::Error::missing_field("type"))?;

                match transaction_type.as_str() {
                    "deposit" => {
                        let amount = amount.ok_or_else(|| de::Error::missing_field("amount"))?;
                        Ok(TransactionRecordType::Deposit { amount })
                    }
                    "withdrawal" => {
                        let amount = amount.ok_or_else(|| de::Error::missing_field("amount"))?;
                        Ok(TransactionRecordType::Withdrawal { amount })
                    }
                    "dispute" => Ok(TransactionRecordType::Dispute),
                    "resolve" => Ok(TransactionRecordType::Resolve),
                    "chargeback" => Ok(TransactionRecordType::Chargeback),
                    _ => Err(de::Error::unknown_variant(
                        &transaction_type,
                        &["deposit", "withdrawal", "dispute", "resolve", "chargeback"],
                    )),
                }
            }
        }

        // Deserialize the struct using the custom visitor
        deserializer.deserialize_any(TransactionTypeVisitor)
    }
}

// Custom implementation used to avoid exposing amount
impl Display for TransactionRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionRecordType::Deposit { .. } => write!(f, "deposit"),
            TransactionRecordType::Withdrawal { .. } => write!(f, "withdrawal"),
            TransactionRecordType::Dispute => write!(f, "dispute"),
            TransactionRecordType::Resolve => write!(f, "resolve"),
            TransactionRecordType::Chargeback => write!(f, "chargeback"),
        }
    }
}

impl Debug for TransactionRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// This reflects the structure of the transaction records in the input CSV file - not used in the engine
#[derive(Deserialize, Clone)]
pub struct TransactionRecord {
    #[serde(flatten)]
    pub tx_type: TransactionRecordType,
    pub client: ClientId,
    pub tx: TransactionId,
}
