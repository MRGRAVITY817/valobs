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
    /// Create a new `Money` instance.
    pub fn new(amount: MoneyAmount, currency: Currency) -> Self {
        Money { amount, currency }
    }

    /// Get the amount of money.
    pub fn amount(&self) -> MoneyAmount {
        self.amount
    }

    /// Get the currency of the money.
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    /// Add two `Money` instances together.
    pub fn add(&self, other: &Money) -> ValobsResult<Money> {
        if self.currency != other.currency {
            return Err("Currencies must be the same".into());
        }
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Subtract one `Money` instance from another.
    pub fn subtract(&self, other: &Money) -> ValobsResult<Money> {
        if self.currency != other.currency {
            return Err("Currencies must be the same".into());
        }
        if self.amount < other.amount {
            return Err("Amount must be greater than or equal to the other amount".into());
        }
        Ok(Money {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Multiply a `Money` instance by a scalar.
    pub fn multiply(&self, scalar: f64) -> Money {
        Money {
            amount: (self.amount as f64 * scalar).round() as MoneyAmount,
            currency: self.currency.clone(),
        }
    }

    /// Divide a `Money` instance by a scalar.
    pub fn divide(&self, scalar: f64) -> ValobsResult<Money> {
        if scalar == 0.0 {
            return Err("Scalar must be greater than 0".into());
        }
        Ok(Money {
            amount: (self.amount as f64 / scalar).round() as MoneyAmount,
            currency: self.currency.clone(),
        })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_money() {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;

        // Act
        let result = Money::new(amount, currency);

        // Assert
        assert_eq!(result.amount(), amount);
        assert_eq!(result.currency(), &currency);
    }
}
