use std::{error, fmt};

use crate::types::value::{Construct, Value};

pub mod value;
#[derive(Debug)]
pub struct SerializeError;

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl error::Error for SerializeError {}
impl serde::ser::Error for SerializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        todo!()
    }
}

pub fn as_value<T>(v: &T) -> Result<Value<'static>, SerializeError>
where
    T: serde::ser::Serialize,
{
    v.serialize(value::Serializer::default())
}
