use std::io;

use serde::de::DeserializeOwned;

use super::codes::FormatCode;
#[derive(Debug, Clone, PartialEq)]
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
    String(String),
    Binary(Vec<u8>),
    Symbol(Symbol),
    List(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Array(Vec<Value>),
}

impl From<Primitive> for Value {
    fn from(val: Primitive) -> Self {
        Value::Primitive(val)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Described {
    pub descriptor: Descriptor,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
    Symbol(Symbol),
    Numeric(u32, u32),
    Reserved(Value),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Primitive(Primitive),
    Described(Box<Described>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub(crate) bytes: Vec<u8>,
}

impl Symbol {
    pub fn new(bytes: Vec<u8>) -> Self {
        Symbol { bytes }
    }
}

