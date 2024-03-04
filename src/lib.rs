pub mod communication;
pub mod financial;
pub mod result;
pub mod temporal;
pub mod traits;

/// Geographical value objects
///
/// This module contains value objects that represent simple geographical data.
/// More than often, these value objects are used in combination with other value objects to represent more complex geographical data.
///
/// ## Example
///
/// ```
/// use valobs::geography::{Latitude, Longitude};
/// use valobs::result::ValobsResult;
///
/// fn main() -> ValobsResult<()> {
///   let latitude = Latitude::new(60.0)?;
///   let longitude = Longitude::new(60.0)?;
///
///   Ok(())
/// }
/// ```
///
/// ## Value objects
///
/// - [Altitude](crate::geography::Altitude)
/// - [Latitude](crate::geography::Latitude)
/// - [Longitude](crate::geography::Longitude)
pub mod geography;

pub mod prelude {
    pub use crate::{
        communication::{Email, PhoneNumber},
        financial::Money,
        temporal::{Date, DateTime, DateTimeTZ, Time},
    };
}
