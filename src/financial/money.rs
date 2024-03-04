/// A value object representing money for simple use cases.
///
/// ## Fields
/// - `amount` - The amount of money.
///    This is a floating point number to allow for fractional amounts.
/// - `currency` - The currency of the money, represented as an ISO 4217 currency code.
pub struct Money {
    amount: MoneyAmount,
    currency: Currency,
}

pub type MoneyAmount = f64;
pub type Currency = String;
