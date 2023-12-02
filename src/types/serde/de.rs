pub mod reader;
pub mod value;
use std::io;

use crate::types::{encoding::de::DecodeErrorKind, value::Value};

use self::reader::ReaderDeserializer;

pub fn from_value<T>(value: Value) -> Result<T, DecodeErrorKind>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(value)
}

pub fn from_reader<T>(reader: impl io::Read) -> Result<T, DecodeErrorKind>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(ReaderDeserializer { reader })
}

pub fn from_slice<T>(slice: &[u8]) -> Result<T, DecodeErrorKind>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(ReaderDeserializer { reader: slice })
}
