use {
    crate::geography::{Altitude, Latitude, Longitude},
    crate::result::ValobsResult,
    serde::{Deserialize, Serialize},
};

/// A geographical location.
///
/// ## What is a _GeoLocation_?
///
/// A geographical location is a point on the Earth's surface.
/// It is defined by a [Latitude](crate::geography::Latitude), a [Longitude](crate::geography::Longitude), and an [Altitude](crate::geography::Altitude).
/// For example, the geographical location of Mount Everest is approximately 27.9881° N, 86.9250° E, and 8,848 meters.
///
/// ## When to use
///
/// Use this type when you want to represent a geographical location.
///
/// ## Limitations
///
/// This type, while serves many use cases, is not suitable for all geographical locations.
///
/// For example, if you need to represent a location of mobile device, you'll also need to include the device's accuracy, speed, and heading. Then you'll need to implement a different type like [this one](https://w3c.github.io/geolocation-api/#coordinates_interface).
///
/// ## Example
///
/// ```
/// use valobs::geography::GeoLocation;
/// use valobs::result::ValobsResult;
///
/// fn main() -> ValobsResult<()> {
///  let location = GeoLocation::new(60.0, 60.0, 1000.0)?;
///  Ok(())
///}
///```
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoLocation {
    latitude: Latitude,
    longitude: Longitude,
    altitude: Altitude,
}

impl GeoLocation {
    /// Creates a new `GeoLocation` instance.
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> ValobsResult<Self> {
        Ok(Self {
            latitude: Latitude::new(latitude)?,
            longitude: Longitude::new(longitude)?,
            altitude: Altitude::new(altitude)?,
        })
    }

    /// Creates a new `GeoLocation` instance without `Altitude`.
    /// This is useful when you don't have altitude information.
    /// For example, when you only have latitude and longitude from a GPS device.
    /// The default altitude will be set to 0 meters.
    pub fn new_without_altitude(latitude: f64, longitude: f64) -> ValobsResult<Self> {
        Self::new(latitude, longitude, 0.0)
    }

    /// Returns the latitude of the geographical location.
    pub fn latitude(&self) -> Latitude {
        self.latitude
    }

    /// Returns the longitude of the geographical location.
    pub fn longitude(&self) -> Longitude {
        self.longitude
    }

    /// Returns the altitude of the geographical location.
    pub fn altitude(&self) -> Altitude {
        self.altitude
    }

    /// Returns the latitude, longitude, and altitude of the geographical location as a tuple
    /// coordinates.
    pub fn coordinates(&self) -> (Latitude, Longitude, Altitude) {
        (self.latitude, self.longitude, self.altitude)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn create_geo_location_with_valid_data() {
        assert!(GeoLocation::new(60.0, 60.0, 1000.0).is_ok());
    }

    #[test]
    fn create_geo_location_with_invalid_latitude() {
        assert!(GeoLocation::new(91.0, 60.0, 1000.0).is_err());
    }

    #[test]
    fn create_geo_location_with_invalid_longitude() {
        assert!(GeoLocation::new(60.0, 181.0, 1000.0).is_err());
    }

    #[test]
    fn create_geo_location_with_invalid_altitude() {
        assert!(GeoLocation::new(60.0, 60.0, 10001.0).is_err());
    }

    #[test]
    fn create_geo_location_without_altitude() {
        assert!(GeoLocation::new_without_altitude(60.0, 60.0).is_ok());
    }

    #[test]
    fn equality_between_two_geo_location_values() -> ValobsResult<()> {
        // Arrange
        let location1 = GeoLocation::new(60.0, 60.0, 1000.0)?;
        let location2 = GeoLocation::new(60.0, 60.0, 1000.0)?;

        // Act
        let result = location1 == location2;

        // Assert
        assert_eq!(result, true);

        Ok(())
    }

    #[test]
    fn inequality_between_two_geo_location_values() -> ValobsResult<()> {
        // Arrange
        let location1 = GeoLocation::new(60.0, 60.0, 1000.0)?;
        let location2 = GeoLocation::new(61.0, 61.0, 1001.0)?;

        // Act
        let result = location1 == location2;

        // Assert
        assert_eq!(result, false);

        Ok(())
    }

    #[test]
    fn returns_latitude_of_geo_location() -> ValobsResult<()> {
        // Arrange
        let location = GeoLocation::new(60.0, 60.0, 1000.0)?;

        // Act
        let result = location.latitude();

        // Assert
        assert_eq!(result, Latitude::new(60.0)?);

        Ok(())
    }

    #[test]
    fn returns_longitude_of_geo_location() -> ValobsResult<()> {
        // Arrange
        let location = GeoLocation::new(60.0, 60.0, 1000.0)?;

        // Act
        let result = location.longitude();

        // Assert
        assert_eq!(result, Longitude::new(60.0)?);

        Ok(())
    }

    #[test]
    fn returns_altitude_of_geo_location() -> ValobsResult<()> {
        // Arrange
        let location = GeoLocation::new(60.0, 60.0, 1000.0)?;

        // Act
        let result = location.altitude();

        // Assert
        assert_eq!(result, Altitude::new(1000.0)?);

        Ok(())
    }

    #[test]
    fn returns_coordinates_of_geo_location() -> ValobsResult<()> {
        // Arrange
        let location = GeoLocation::new(60.0, 60.0, 1000.0)?;

        // Act
        let result = location.coordinates();

        // Assert
        assert_eq!(
            result,
            (
                Latitude::new(60.0)?,
                Longitude::new(60.0)?,
                Altitude::new(1000.0)?
            )
        );

        Ok(())
    }

    #[test]
    fn serialize_to_json() -> ValobsResult<()> {
        // Arrange
        let location = GeoLocation::new(60.0, 60.0, 1000.0)?;

        // Act
        let result = serde_json::to_string(&location).unwrap();

        // Assert
        assert_eq!(
            result,
            r#"{"latitude":60.0,"longitude":60.0,"altitude":1000.0}"#
        );

        Ok(())
    }

    #[test]
    fn deserialize_from_json() -> ValobsResult<()> {
        // Arrange
        let location = GeoLocation::new(60.0, 60.0, 1000.0)?;
        let json = r#"{"latitude":60.0,"longitude":60.0,"altitude":1000.0}"#;

        // Act
        let result: GeoLocation = serde_json::from_str(json).unwrap();

        // Assert
        assert_eq!(result, location);

        Ok(())
    }
}
