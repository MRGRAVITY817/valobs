use serde::{Deserialize, Serialize};

pub trait ValueObject<'de>: PartialEq + Eq + Serialize + Deserialize<'de> {}
