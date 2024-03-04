use crate::result::ValobsResult;
use serde::{Deserialize, Serialize};

pub trait ValobsValidate {
    type Target;

    fn validate(value: impl Into<Self::Target>) -> ValobsResult<Self>
    where
        Self: Sized;
}

pub trait ValueObject<'de>: ValobsValidate + PartialEq + Eq + Serialize + Deserialize<'de> {}
