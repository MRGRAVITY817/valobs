use super::currency::Currency;
use crate::{result::ValobsResult, traits::ValueObject};
use serde::{Deserialize, Serialize};

/// A value object representing money for simple use cases.
///
/// ## Fields
/// - `amount` - The amount of money.
/// - `currency` - The currency of the money, represented as an ISO 4217 currency code.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub struct Money {
    amount: MoneyAmount,
    currency: Currency,
}

impl Money {
    /// Create a new `Money` instance.
    pub fn new(amount: MoneyAmount, currency: Currency) -> ValobsResult<Money> {
        if amount == 0 {
            return Err("Amount must be greater than 0".into());
        }
        Ok(Money { amount, currency })
    }
    /// Get the amount of money.
    pub fn amount(&self) -> MoneyAmount {
        self.amount
    }

    /// Get the currency of the money.
    pub fn currency(&self) -> &Currency {
        &self.currency
    }

    pub fn check_currency(&self, other: &Money) -> ValobsResult<()> {
        if self.currency != other.currency {
            return Err("Currencies must be the same".into());
        }
        Ok(())
    }

    /// Add two `Money` instances together.
    pub fn add(&self, other: &Money) -> ValobsResult<Money> {
        self.check_currency(other)?;
        Ok(Money {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Subtract one `Money` instance from another.
    pub fn subtract(&self, other: &Money) -> ValobsResult<Money> {
        if self.amount < other.amount {
            return Err("Amount must be greater than or equal to the other amount".into());
        }
        self.check_currency(other)?;
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

    /// Allocate the `Money` instance between a number of parts.
    ///
    /// The result is an array of `Money` instances, where the sum of the parts equals the original amount.
    /// The first parts will be larger than the last parts if there is a remainder.
    ///
    /// ## When to use
    /// This is useful for **splitting a bill** between a number of people.
    ///
    /// ## Example
    /// ```
    /// use valobs::financial::{Currency, Money};
    /// use valobs::result::ValobsResult;
    ///
    /// fn main() -> ValobsResult<()> {
    ///    let money = Money::new(100, Currency::USD)?;
    ///    let parts = 3;
    ///
    ///    // Split 100 USD between 3 people
    ///    let result = money.allocate(parts)?;
    ///
    ///    assert_eq!(result.len(), 3);
    ///    assert_eq!(result[0].amount(), 34);
    ///    assert_eq!(result[1].amount(), 33);
    ///    assert_eq!(result[2].amount(), 33);
    ///    assert_eq!(result[0].currency(), &Currency::USD);
    ///    assert_eq!(result[1].currency(), &Currency::USD);
    ///    assert_eq!(result[2].currency(), &Currency::USD);
    ///    Ok(())
    /// }
    /// ```
    pub fn allocate(&self, parts: u32) -> ValobsResult<Vec<Money>> {
        if parts == 0 {
            return Err("Parts must be greater than 0".into());
        }
        let low_result = Self::new(self.amount / parts as MoneyAmount, self.currency)?;
        let remainder = self.amount % parts as MoneyAmount;

        (0..parts as MoneyAmount)
            .map(|i| {
                if i < remainder {
                    Self::new(low_result.amount + 1, self.currency.clone())
                } else {
                    Ok(low_result)
                }
            })
            .collect::<ValobsResult<Vec<Money>>>()
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
        assert!(result.is_ok());
    }

    #[test]
    fn fails_to_create_money_when_amount_is_zero() {
        // Arrange
        let amount: MoneyAmount = 0;
        let currency = Currency::USD;

        // Act
        let result = Money::new(amount, currency);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn add_money() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let other = Money::new(50, currency)?;

        // Act
        let result = money.add(&other).unwrap();

        // Assert
        assert_eq!(result.amount(), 150);
        assert_eq!(result.currency(), &currency);

        Ok(())
    }

    #[test]
    fn subtract_money() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let other = Money::new(50, currency)?;

        // Act
        let result = money.subtract(&other).unwrap();

        // Assert
        assert_eq!(result.amount(), 50);
        assert_eq!(result.currency(), &currency);

        Ok(())
    }

    #[test]
    fn fails_to_subtract_money_when_given_amout_is_larger() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let other = Money::new(150, currency)?;

        // Act
        let result = money.subtract(&other);

        // Assert
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn multiply_money() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let scalar = 2.0;

        // Act
        let result = money.multiply(scalar);

        // Assert
        assert_eq!(result.amount(), 200);
        assert_eq!(result.currency(), &currency);

        Ok(())
    }

    #[test]
    fn divide_money() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let scalar = 2.0;

        // Act
        let result = money.divide(scalar)?;

        // Assert
        assert_eq!(result.amount(), 50);
        assert_eq!(result.currency(), &currency);

        Ok(())
    }

    #[test]
    fn fails_to_divide_money_when_scalar_is_zero() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let scalar = 0.0;

        // Act
        let result = money.divide(scalar);

        // Assert
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn allocate_money() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let parts = 3;

        // Act
        let result = money.allocate(parts)?;

        // Assert
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].amount(), 34);
        assert_eq!(result[1].amount(), 33);
        assert_eq!(result[2].amount(), 33);
        assert_eq!(result[0].currency(), &currency);
        assert_eq!(result[1].currency(), &currency);
        assert_eq!(result[2].currency(), &currency);

        Ok(())
    }

    #[test]
    fn fails_to_allocate_money_when_parts_is_zero() -> ValobsResult<()> {
        // Arrange
        let amount: MoneyAmount = 100;
        let currency = Currency::USD;
        let money = Money::new(amount, currency)?;
        let parts = 0;

        // Act
        let result = money.allocate(parts);

        // Assert
        assert!(result.is_err());

        Ok(())
    }
}
