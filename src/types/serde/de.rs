pub mod reader;
pub mod value;
use std::io;

use crate::types::{encoding::de::slice::View, value::Value};

use self::reader::ReaderDeserializer;
#[derive(Debug)]
pub struct DeserializeError(io::Error);

impl DeserializeError {
    pub fn into_inner(self) -> io::Error {
        return self.0;
    }
}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for DeserializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        io::Error::other(msg.to_string()).into()
    }
}

impl From<io::Error> for DeserializeError {
    fn from(err: io::Error) -> Self {
        DeserializeError(err)
    }
}

impl Into<io::Error> for DeserializeError {
    fn into(self) -> io::Error {
        self.0
    }
}

pub fn from_value<T>(value: Value) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(value).map_err(Into::into)
}

pub fn from_reader<T>(reader: impl io::Read) -> io::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    T::deserialize(ReaderDeserializer { reader }).map_err(Into::into)
}

pub fn from_slice<'de, T>(mut slice: &'de [u8]) -> io::Result<T>
where
    T: serde::de::Deserialize<'de>,
{
    T::deserialize(Value::view(&mut slice)?).map_err(Into::into)
}
