use std::{borrow::Cow, io, ops::DerefMut};

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
    Binary(Binary<'a>),
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Binary<'a>(pub(crate) Cow<'a, [u8]>);

impl<'a> Binary<'a> {
    pub fn new(bin: impl Into<Cow<'a, [u8]>>) -> Self {
        Binary(bin.into())
    }
}

impl<'a> std::ops::Deref for Binary<'a> {
    type Target = Cow<'a, [u8]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Binary<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

pub trait Construct {
    fn constructor() -> Constructor<'static>;
}

macro_rules! derive_primitives {
    ($($ty:ty => $code:expr),*) => {
        $(
            impl Construct for $ty {
                fn constructor() -> Constructor<'static> {
                    Constructor::FormatCode($code)
                }
            }
        )*
    };
}

derive_primitives! {
    bool => FormatCode::BOOLEAN,
    u8 => FormatCode::UBYTE,
    u16 => FormatCode::USHORT,
    u32 => FormatCode::UINT,
    u64 => FormatCode::ULONG,
    i8 => FormatCode::BYTE,
    i16 => FormatCode::SHORT,
    i32 => FormatCode::INT,
    i64 => FormatCode::LONG,
    f32 => FormatCode::FLOAT,
    f64 => FormatCode::DOUBLE,
    char => FormatCode::CHAR,
    String => FormatCode::STRING32_UTF8,
    Binary<'_> => FormatCode::BINARY32,
    Symbol<'_> => FormatCode::SYMBOL32
}

impl<const N: usize, T: Construct> Construct for [T; N] {
    fn constructor() -> Constructor<'static> {
        Constructor::FormatCode(FormatCode::ARRAY32)
    }
}

impl<T: Construct> Construct for Vec<T> {
    fn constructor() -> Constructor<'static> {
        Constructor::FormatCode(FormatCode::ARRAY32)
    }
}