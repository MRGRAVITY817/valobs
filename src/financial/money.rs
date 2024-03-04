use super::currency::Currency;
use crate::{
    result::ValobsResult,
    traits::{ValobsValidate, ValueObject},
};
use serde::{Deserialize, Serialize};

/// A value object representing money for simple use cases.
///
/// ## Fields
/// - `amount` - The amount of money.
/// - `currency` - The currency of the money, represented as an ISO 4217 currency code.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    amount: MoneyAmount,
    currency: Currency,
}

impl Money {
    pub fn new(amount: MoneyAmount, currency: Currency) -> Self {
        Money { amount, currency }
    }
}

impl ValobsValidate for Money {
    type Target = (MoneyAmount, Currency);

    fn validate(value: impl Into<Self::Target>) -> ValobsResult<Self> {
        let (amount, currency) = value.into();
        if amount == 0 {
            return Err("Amount must be greater than 0".into());
        }
        Ok(Money { amount, currency })
    }
}

impl ValueObject<'_> for Money {}

pub type MoneyAmount = u64;
