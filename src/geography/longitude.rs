use crate::result::ValobsResult;

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
/// It can be used alone, or as part of a larger value object, such as a `GeoLocation`, with a [`Latitude`].
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
pub struct Longitude(f64);

impl Longitude {
    /// Create a new `Longitude` instance.
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
