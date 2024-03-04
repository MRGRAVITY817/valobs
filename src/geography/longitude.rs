use crate::result::ValobsResult;
use serde::{Deserialize, Serialize};

/// A value object representing a longitude.
///
/// ## What is a _Longitude_?
///
/// Longitude is a geographic coordinate that specifies the east-west position of a point on the Earth's surface.
/// For example, the longitude of the Prime Meridian (which runs through Greenwich, London) is 0 degrees, and the longitude of the International Date Line is 180 degrees.
///
/// ## When to use
///
/// Use this type when you want to represent a longitude.
/// It can be used alone, or as part of a larger value object, such as a `GeoLocation`, with a [Latitude].
///
/// # Example
/// ```
/// use valobs::geography::Longitude;
/// use valobs::result::ValobsResult;
///
/// fn main() -> ValobsResult<()> {
///   let longitude = Longitude::new(60.0)?;
///
///   Ok(())
/// }
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Longitude(f64);

impl Longitude {
    /// Creates a new `Longitude` instance.
    pub fn new(longitude: f64) -> ValobsResult<Self> {
        if longitude < -180.0 || longitude > 180.0 {
            return Err("Longitude must be between -180 and 180".into());
        }
        if longitude.is_nan() {
            return Err("Longitude must not be NaN".into());
        }
        if longitude.is_infinite() {
            return Err("Longitude must not be infinite".into());
        }

        Ok(Self(longitude))
    }
}

impl AsRef<f64> for Longitude {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_longitude_with_valid_data() {
        assert!(Longitude::new(60.0).is_ok());
        assert!(Longitude::new(-180.0).is_ok());
        assert!(Longitude::new(180.0).is_ok());
    }

    #[test]
    fn fails_to_create_longitude_with_invalid_data() {
        assert!(Longitude::new(180.1).is_err());
        assert!(Longitude::new(-180.1).is_err());
        assert!(Longitude::new(f64::NAN).is_err());
        assert!(Longitude::new(f64::INFINITY).is_err());
        assert!(Longitude::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn equality_between_two_longitude_values() -> ValobsResult<()> {
        // Arrange
        let longitude1 = Longitude::new(60.0)?;
        let longitude2 = Longitude::new(60.0)?;

        // Act
        let result = longitude1 == longitude2;

        // Assert
        assert!(result);
        Ok(())
    }

    #[test]
    fn inequality_between_two_longitude_values() -> ValobsResult<()> {
        // Arrange
        let longitude1 = Longitude::new(60.0)?;
        let longitude2 = Longitude::new(61.0)?;

        // Act
        let result = longitude1 == longitude2;

        // Assert
        assert!(!result);
        Ok(())
    }

    #[test]
    fn as_ref_returns_a_reference_to_the_internal_value() -> ValobsResult<()> {
        // Arrange
        let longitude = Longitude::new(60.0)?;

        // Act
        let result = longitude.as_ref();

        // Assert
        assert_eq!(*result, 60.0);
        Ok(())
    }

    #[test]
    fn serialize_to_json() -> ValobsResult<()> {
        // Arrange
        let longitude = Longitude::new(60.0)?;

        // Act
        let result = serde_json::to_string(&longitude).unwrap();

        // Assert
        assert_eq!(result, "60.0");
        Ok(())
    }

    #[test]
    fn deserialize_from_json() -> ValobsResult<()> {
        // Arrange
        let longitude = Longitude::new(60.0)?;
        let json = serde_json::to_string(&longitude).unwrap();

        // Act
        let result: Longitude = serde_json::from_str(&json).unwrap();

        // Assert
        assert_eq!(result, longitude);
        Ok(())
    }
}
