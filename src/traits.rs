use crate::result::ValobsResult;
use serde::{Deserialize, Serialize};

pub trait Validate {
    type Target;

    fn validate(value: Self::Target) -> ValobsResult<Self::Target>;
}

pub trait ValueObject<'de>: Validate + PartialEq + Eq + Serialize + Deserialize<'de> {}
