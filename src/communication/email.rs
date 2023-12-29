use {
    crate::{
        result::ValobsResult,
        traits::{Validate, ValueObject},
    },
    lazy_static::lazy_static,
    regex::Regex,
    serde::{Deserialize, Serialize},
};

lazy_static! {
    static ref EMAIL_REGEX: Regex =
        Regex::new("[A-Z0-9a-z._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,64}").unwrap();
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Validate for Email {
    type Target = Email;

    fn validate(value: Self::Target) -> ValobsResult<Self::Target> {
        if EMAIL_REGEX.is_match(&value.0) {
            return Ok(value);
        }

        Err("Nope".into())
    }
}

impl<'de> ValueObject<'de> for Email {}
