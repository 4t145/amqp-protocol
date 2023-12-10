use std::str::Utf8Error;

use bytes::Bytes;
use serde::Deserialize;

use crate::{codec::Decode, constructor::Constructor, value::Value};

use super::codes::FormatCode;
#[derive(Debug, Clone)]
pub enum Primitive {
    Null,
    Boolean(bool),
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Decimal32(),
    Decimal64(),
    Decimal128(),
    Char(char),
    Timestamp(u64),
    Uuid([u8; 16]),
    String(AmqpString),
    Binary(Binary),
    Symbol(Symbol),
    List(AmqpList),
    Map(AmqpMap),
    Array(AmqpArray),
}

#[derive(Debug, Clone)]
pub struct AmqpString(pub(crate) Bytes);

impl AmqpString {
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.0.as_ref())
    }
    pub fn from_static_str(str: &'static str) -> Self {
        AmqpString(Bytes::from(str))
    }
    pub fn from_string(str: String) -> Self {
        AmqpString(Bytes::from(str))
    }
}

#[derive(Debug, Clone)]

pub struct Binary(pub(crate) Bytes);

#[derive(Debug, Clone)]
pub struct Symbol(pub(crate) Bytes);

#[derive(Debug, Clone)]
pub struct AmqpArray {
    pub(crate) constructor: Constructor,
    pub(crate) count: usize,
    pub(crate) data: Bytes,
}

impl Iterator for AmqpArray {
    type Item = Primitive;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            self.constructor.construct(&mut self.data)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmqpList {
    pub(crate) count: usize,
    pub(crate) data: Bytes,
}

impl Iterator for AmqpList {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            Value::decode(&mut self.data)
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AmqpMap {
    pub(crate) count: usize,
    pub(crate) data: Bytes,
}

impl Iterator for AmqpMap {
    type Item = (Value, Value);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            Some((
                Value::decode(&mut self.data)?,
                Value::decode(&mut self.data)?,
            ))
        }
    }
}
