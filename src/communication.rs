use lazy_static::lazy_static;
use nutype::nutype;
use regex::Regex;

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new("[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,64}").unwrap();
}

#[nutype(
    sanitize(trim, lowercase),
    validate(
        len_char_min = 5,
        len_char_max = 30,
        regex = EMAIL_REGEX,
    ),
    derive(Debug, PartialEq, AsRef),
)]
pub struct Email(String);

pub use phonenumber::PhoneNumber;
