/// A value object representing money for simple use cases.
///
/// ## Fields
/// - `amount` - The amount of money.
/// - `currency` - The currency of the money, represented as an ISO 4217 currency code.
pub struct Money {
    amount: MoneyAmount,
    currency: Currency,
}

pub type MoneyAmount = u64;
pub type Currency = String;
