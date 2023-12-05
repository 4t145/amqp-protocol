use std::{borrow::Cow, io};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::codes::FormatCode;
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Primitive<'a> {
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
    String(Cow<'a, str>),
    Binary(Cow<'a, [u8]>),
    Symbol(Symbol<'a>),
    List(Vec<Value<'a>>),
    Map(Vec<(Value<'a>, Value<'a>)>),
    Array(Vec<Value<'a>>),
}

impl<'a> From<Primitive<'a>> for Value<'a> {
    fn from(val: Primitive<'a>) -> Self {
        Value::Primitive(val)
    }
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Described<'a> {
    pub descriptor: Descriptor<'a>,
    pub value: Value<'a>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Descriptor<'a> {
    Symbol(Symbol<'a>),
    Numeric(u32, u32),
    Reserved(Value<'a>),
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Value<'a> {
    Primitive(Primitive<'a>),
    Described(Box<Described<'a>>),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol<'a> {
    pub(crate) bytes: Cow<'a, [u8]>,
}

impl<'a> Symbol<'a> {
    pub fn new(bytes: impl Into<Cow<'a, [u8]>>) -> Self {
        Symbol {
            bytes: bytes.into(),
        }
    }
}
#[derive(Debug)]
pub enum Constructor<'a> {
    FormatCode(FormatCode),
    Described {
        descriptor: Descriptor<'a>,
        constructor: Box<Constructor<'a>>,
    },
}
