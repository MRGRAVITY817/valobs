use crate::result::ValobsResult;
use serde::{Deserialize, Serialize};

/// A value object representing a latitude.
///
/// ## What is a _Latitude_?
///
/// Latitude is a geographic coordinate that specifies the north-south position of a point on the Earth's surface.
/// For example, the latitude of the North Pole is 90 degrees, and the latitude of the South Pole is -90 degrees.
///
/// ## When to use
///
/// Use this type when you want to represent a latitude.
/// It can be used alone, or as part of a larger value object, such as a `GeoLocation`, with a `Longitude`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Latitude(f64);

impl Latitude {
    /// Create a new `Latitude` instance.
    ///
    /// # Example
    /// ```
    /// use valobs::geography::Latitude;
    /// use valobs::result::ValobsResult;
    ///
    /// fn main() -> ValobsResult<()> {
    ///    let latitude = Latitude::new(60.0)?;
    ///    Ok(())
    /// }
    /// ```
    pub fn new(latitude: f64) -> ValobsResult<Self> {
        if latitude < -90.0 || latitude > 90.0 {
            return Err("Latitude must be between -90 and 90".into());
        }
        if latitude.is_nan() {
            return Err("Latitude must not be NaN".into());
        }
        if latitude.is_infinite() {
            return Err("Latitude must not be infinite".into());
        }

        Ok(Self(latitude))
    }
}

impl AsRef<f64> for Latitude {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test valid latitudes
        assert!(Latitude::new(60.0).is_ok());
        assert!(Latitude::new(-90.0).is_ok());
        assert!(Latitude::new(90.0).is_ok());

        // Test invalid latitudes
        assert!(Latitude::new(90.1).is_err());
        assert!(Latitude::new(-90.1).is_err());
        assert!(Latitude::new(f64::NAN).is_err());
        assert!(Latitude::new(f64::INFINITY).is_err());
        assert!(Latitude::new(f64::NEG_INFINITY).is_err());
    }
}
