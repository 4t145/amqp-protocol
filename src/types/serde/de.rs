pub mod reader;
pub mod value;
use crate::types::{encoding::de::DecodeErrorKind, value::Value};

pub fn from_value<T>(value: Value) -> Result<T, DecodeErrorKind>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(value)
}
