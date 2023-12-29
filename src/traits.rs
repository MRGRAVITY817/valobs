use crate::result::ValobsResult;
use serde::{Deserialize, Serialize};

pub trait Validate {
    type Target;

    fn validate(value: impl Into<Self::Target>) -> ValobsResult<Self>
    where
        Self: Sized;
}

pub trait ValueObject<'de>: Validate + PartialEq + Eq + Serialize + Deserialize<'de> {}
