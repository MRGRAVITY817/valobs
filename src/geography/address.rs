use {
    super::Country,
    serde::{Deserialize, Serialize},
};

/// A value object representing a physical address.
///
/// ## What is an _address_?
///
/// An address is a collection of information that identifies the location of a person or an entity.
/// It typically includes the street name and number, city, state, postal code, and country.
/// In some cases, it may also include additional information such as the apartment number or the
/// name of the recipient.
///
/// ## Limitations
///
/// Though this implementation is a good starting point, and useful for general applications, it has some limitations. For example, it does not support international addresses, which may have different formats and requirements.
///
/// It also does not separate too much details about the address, such as the name of the recipient,
/// the apartment number, or the type of address (e.g., residential, business, etc.). But that can
/// be easily added by extending the `Address` struct.
///
/// ## Examples
///
/// ```
/// use valobs::geography::{Address, Country};
///
/// let address = Address {
///    street: "123 Main St",
///    city: "Anytown",
///    state: "NY",
///    postal_code: "12345",
///    country: Country::USA,
/// };
///
/// assert_eq!(address.street, "123 Main St");
/// assert_eq!(address.city, "Anytown");
/// assert_eq!(address.state, "NY");
/// assert_eq!(address.postal_code, "12345");
/// assert_eq!(address.country, Country::USA);
/// ```
///
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Address<'a> {
    pub street: &'a str,
    pub city: &'a str,
    pub state: &'a str,
    pub postal_code: &'a str,
    pub country: Country,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_address_with_valid_data() {
        let address = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };

        assert_eq!(address.street, "123 Main St");
        assert_eq!(address.city, "Anytown");
        assert_eq!(address.state, "NY");
        assert_eq!(address.postal_code, "12345");
        assert_eq!(address.country, Country::USA);
    }

    #[test]
    fn create_address_with_invalid_data() {
        let address = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };

        assert_ne!(address.street, "123 Main St ");
        assert_ne!(address.city, "Anytown ");
        assert_ne!(address.state, "NY ");
        assert_ne!(address.postal_code, "12345 ");
        assert_ne!(address.country, Country::CAN);
    }

    #[test]
    fn equality() {
        let address1 = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };
        let address2 = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };

        assert_eq!(address1, address2);
    }

    #[test]
    fn inequality() {
        let address1 = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };
        let address2 = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::CAN,
        };

        assert_ne!(address1, address2);
    }

    #[test]
    fn serialize_address_to_json() {
        let address = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };

        let result = serde_json::to_string(&address).unwrap();

        assert_eq!(
            result,
            r#"{"street":"123 Main St","city":"Anytown","state":"NY","postal_code":"12345","country":"USA"}"#
        );
    }

    #[test]
    fn deserialize_address_from_json() {
        let address = Address {
            street: "123 Main St",
            city: "Anytown",
            state: "NY",
            postal_code: "12345",
            country: Country::USA,
        };

        let result = serde_json::from_str::<Address>(
            r#"{"street":"123 Main St","city":"Anytown","state":"NY","postal_code":"12345","country":"USA"}"#,
        )
        .unwrap();

        assert_eq!(result, address);
    }
}
