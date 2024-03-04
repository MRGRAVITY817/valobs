use crate::result::ValobsResult;
use serde::{Deserialize, Serialize};

/// A value object representing an altitude.
///
/// ## What is an _Altitude_?
///
/// Altitude is the height of an object or point in relation to a fixed reference point, such as the Earth's sea level.
/// For example, the altitude of Mount Everest is 8,848 meters.
///
/// For more information, see [Wikipedia](https://en.wikipedia.org/wiki/Altitude).
///
/// ## When to use
///
/// Use this type when you want to represent an altitude.
///
/// Altitude is often used in aviation and geography to describe the height of an object or point above the Earth's surface.
/// It is also used in meteorology to describe the height of a weather balloon or aircraft above the Earth's surface.
///
/// ## Limitations
///
/// The altitude must be between -1000 and 10000 meters.
/// This is a common limitation in aviation and meteorology, as most aircraft and weather balloons operate within this range.
///
/// If you need to represent altitudes outside this range, you should use a different type, such as `f64`.
///
/// ## Example
///
/// ```
/// use valobs::geography::Altitude;
/// use valobs::result::ValobsResult;
///
/// fn main() -> ValobsResult<()> {
///   let altitude = Altitude::new(1000.0)?;
///
///   Ok(())
/// }
/// ```
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct Altitude(f64);

impl Altitude {
    /// Creates a new `Altitude` instance.
    pub fn new(altitude: f64) -> ValobsResult<Self> {
        if altitude < -1000.0 || altitude > 10000.0 {
            return Err("Altitude must be between -1000 and 10000".into());
        }
        if altitude.is_nan() {
            return Err("Altitude must not be NaN".into());
        }
        if altitude.is_infinite() {
            return Err("Altitude must not be infinite".into());
        }

        Ok(Self(altitude))
    }
}

impl AsRef<f64> for Altitude {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_altitude_with_valid_data() {
        assert!(Altitude::new(1000.0).is_ok());
        assert!(Altitude::new(-1000.0).is_ok());
        assert!(Altitude::new(10000.0).is_ok());
    }

    #[test]
    fn fails_to_create_altitude_with_invalid_data() {
        assert!(Altitude::new(10000.1).is_err());
        assert!(Altitude::new(-1000.1).is_err());
        assert!(Altitude::new(f64::NAN).is_err());
        assert!(Altitude::new(f64::INFINITY).is_err());
        assert!(Altitude::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn equality_between_two_altitude_values() -> ValobsResult<()> {
        // Arrange
        let altitude1 = Altitude::new(1000.0)?;
        let altitude2 = Altitude::new(1000.0)?;

        // Act
        let result = altitude1 == altitude2;

        // Assert
        assert_eq!(result, true);

        Ok(())
    }

    #[test]
    fn as_ref_returns_a_reference_to_the_inner_value() -> ValobsResult<()> {
        // Arrange
        let altitude = Altitude::new(1000.0)?;

        // Act
        let result = altitude.as_ref();

        // Assert
        assert_eq!(result, &1000.0);

        Ok(())
    }

    #[test]
    fn inequality_between_two_altitude_values() -> ValobsResult<()> {
        // Arrange
        let altitude1 = Altitude::new(1000.0)?;
        let altitude2 = Altitude::new(1001.0)?;

        // Act
        let result = altitude1 == altitude2;

        // Assert
        assert_eq!(result, false);

        Ok(())
    }

    #[test]
    fn inner_value_with_as_ref() -> ValobsResult<()> {
        // Arrange
        let altitude = Altitude::new(1000.0)?;

        // Act
        let result = altitude.as_ref();

        // Assert
        assert_eq!(*result, 1000.0);

        Ok(())
    }

    #[test]
    fn serialize_altitude_to_json() -> ValobsResult<()> {
        // Arrange
        let altitude = Altitude::new(1000.0)?;

        // Act
        let result = serde_json::to_string(&altitude).unwrap();

        // Assert
        assert_eq!(result, "1000.0");

        Ok(())
    }

    #[test]
    fn deserialize_altitude_from_json() -> ValobsResult<()> {
        // Arrange
        let altitude = Altitude::new(1000.0)?;

        // Act
        let result = serde_json::from_str::<Altitude>("1000.0").unwrap();

        // Assert
        assert_eq!(result, altitude);

        Ok(())
    }
}
