use serde::{Deserialize, Serialize};

/// A value object representing a continent.
///
/// ## What is a _Continent_?
///
/// A continent is one of several very large landmasses. Generally identified by convention rather than any strict criteria, up to seven regions are commonly regarded as continents.
///
/// Ordered from largest in size to smallest, they are: Asia, Africa, North America, South America, Antarctica, Europe, and Australia.
///
/// ## When to use
///
/// Use this type when you want to represent a continent.
/// For example, when you want to represent the continent of a country or city.
/// It's often used in geography and travel to describe the location of a place.
///
/// ## Example
///
/// ```
/// use valobs::geography::Continent;
/// use valobs::result::ValobsResult;
///
/// fn main() -> ValobsResult<()> {
///  let africa = Continent::Africa;
///
///  Ok(())
/// }
///
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Continent {
    Africa,
    Antarctica,
    Asia,
    Europe,
    NorthAmerica,
    Oceania,
    SouthAmerica,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_continent_with_valid_data() {
        assert_eq!(Continent::Africa, Continent::Africa);
        assert_eq!(Continent::Antarctica, Continent::Antarctica);
        assert_eq!(Continent::Asia, Continent::Asia);
        assert_eq!(Continent::Europe, Continent::Europe);
        assert_eq!(Continent::NorthAmerica, Continent::NorthAmerica);
        assert_eq!(Continent::Oceania, Continent::Oceania);
        assert_eq!(Continent::SouthAmerica, Continent::SouthAmerica);
    }

    #[test]
    fn create_continent_with_invalid_data() {
        assert_ne!(Continent::Africa, Continent::Antarctica);
        assert_ne!(Continent::Africa, Continent::Asia);
        assert_ne!(Continent::Africa, Continent::Europe);
        assert_ne!(Continent::Africa, Continent::NorthAmerica);
        assert_ne!(Continent::Africa, Continent::Oceania);
        assert_ne!(Continent::Africa, Continent::SouthAmerica);
    }

    #[test]
    fn equality() {
        assert_eq!(Continent::Africa, Continent::Africa);
        assert_ne!(Continent::Africa, Continent::Antarctica);
        assert_ne!(Continent::Africa, Continent::Asia);
        assert_ne!(Continent::Africa, Continent::Europe);
        assert_ne!(Continent::Africa, Continent::NorthAmerica);
        assert_ne!(Continent::Africa, Continent::Oceania);
        assert_ne!(Continent::Africa, Continent::SouthAmerica);
    }

    #[test]
    fn inequality() {
        assert_ne!(Continent::Africa, Continent::Antarctica);
        assert_ne!(Continent::Africa, Continent::Asia);
        assert_ne!(Continent::Africa, Continent::Europe);
        assert_ne!(Continent::Africa, Continent::NorthAmerica);
        assert_ne!(Continent::Africa, Continent::Oceania);
        assert_ne!(Continent::Africa, Continent::SouthAmerica);
    }

    #[test]
    fn serialize_continent_to_json() {
        // Arrange
        let continent = Continent::Africa;

        // Act
        let result = serde_json::to_string(&continent).unwrap();

        // Assert
        assert_eq!(result, "\"Africa\"");
    }

    #[test]
    fn deserialize_continent_from_json() {
        // Arrange
        let continent = Continent::Africa;

        // Act
        let result = serde_json::from_str::<Continent>("\"Africa\"").unwrap();

        // Assert
        assert_eq!(result, continent);
    }
}
