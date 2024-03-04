use serde::{Deserialize, Serialize};

pub trait ValueObject<'de>: PartialEq + Serialize + Deserialize<'de> {}
