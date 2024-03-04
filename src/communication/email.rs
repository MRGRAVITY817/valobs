use {
    crate::{
        result::ValobsResult,
        traits::{ValobsValidate, ValueObject},
    },
    lazy_static::lazy_static,
    nutype::nutype,
    regex::Regex,
};

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new("[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,64}").unwrap();
}

#[nutype(
    sanitize(trim, lowercase),
    validate(
        len_char_min = 5,
        len_char_max = 20,
        regex = EMAIL_REGEX,
    ),
    derive(Debug, PartialEq, Eq, AsRef, Serialize, Deserialize),
)]
pub struct Email(String);

impl ValobsValidate for Email {
    type Target = String;

    fn validate(value: impl Into<Self::Target>) -> ValobsResult<Self> {
        Email::new(value).map_err(|e| e.to_string())
    }
}

impl<'de> ValueObject<'de> for Email {}
