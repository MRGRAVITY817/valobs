mod country_code;
mod country_name;

pub use self::{country_code::CountryCode, country_name::CountryName};

pub struct Country {
    pub name: CountryName,
    pub code: CountryCode,
}
