use std::{io, string};

use super::{
    reader::{size1, size4, Decode},
    ArrayIter, CompoundIter,
};
use crate::types::{
    codes::FormatCode,
    value::{Constructor, Described, Descriptor, Primitive, Symbol, Value},
};
use bytes::Bytes;

#[derive(Debug)]
pub enum DecodeErrorKind {
    Expect(&'static str),
    InvalidFormatCode(&'static str, u8),
    InvalidChar(u32),
    FromUtf8(string::FromUtf8Error),
    Custom(String),
}
pub type DecodeResult<T> = io::Result<T>;

pub trait View<'a>: Sized + 'a {
    fn view(buffer: &mut &'a [u8]) -> io::Result<Self>;
}

impl<'a> View<'a> for Value<'a> {
    fn view(buffer: &mut &'a [u8]) -> io::Result<Self> {
        let constructor = dbg!(Constructor::view(buffer)?);
        dbg!(constructor.construct_slice(buffer))
    }
}

impl<'a> View<'a> for Descriptor<'a> {
    fn view(buffer: &mut &'a [u8]) -> io::Result<Self> {
        let constructor = Constructor::view(buffer)?;
        let descriptor = constructor.construct_slice(buffer)?;
        match descriptor {
            Value::Primitive(Primitive::Symbol(symbol)) => Ok(Descriptor::Symbol(symbol)),
            Value::Primitive(Primitive::ULong(bytes)) => {
                let domain_id = (bytes >> 32) as u32;
                let descriptor_id = bytes as u32;
                Ok(Descriptor::Numeric(domain_id, descriptor_id))
            }
            reserved => Ok(Descriptor::Reserved(reserved)),
        }
    }
}

impl<'a> View<'a> for Constructor<'a> {
    fn view(buffer: &mut &'a [u8]) -> io::Result<Self> {
        let byte = u8::decode(buffer)?;
        match byte {
            0x00 => {
                let descriptor = Descriptor::view(buffer)?;
                let constructor = Constructor::view(buffer)?;
                Ok(Constructor::Described {
                    descriptor,
                    constructor: Box::new(constructor),
                })
            }
            code if code & 0x0f != 0x0f => Ok(Constructor::FormatCode(FormatCode::Primitive(code))),
            code => {
                let ext = u8::decode(buffer)?;
                Ok(Constructor::FormatCode(FormatCode::Ext(code, ext)))
            }
        }
    }
}

impl<'a> Constructor<'a> {
    pub fn construct_slice(&self, buffer: &mut &'a [u8]) -> io::Result<Value<'a>> {
        let value = match self {
            Constructor::FormatCode(code) => match *code {
                FormatCode::NULL => Primitive::Null,
                FormatCode::BOOLEAN_TRUE => Primitive::Boolean(true),
                FormatCode::BOOLEAN_FALSE => Primitive::Boolean(false),
                FormatCode::UINT_0 => Primitive::UInt(0),
                FormatCode::ULONG_0 => Primitive::ULong(0),
                FormatCode::LIST0 => Primitive::List(vec![]),
                FormatCode::BOOLEAN => Primitive::Boolean(0 == u8::decode(buffer)?),
                FormatCode::UBYTE => Primitive::UByte(u8::decode(buffer)?),
                FormatCode::BYTE => Primitive::Byte(i8::decode(buffer)?),
                FormatCode::SMALL_UINT => Primitive::UInt(u8::decode(buffer)? as u32),
                FormatCode::SMALL_ULONG => Primitive::ULong(u8::decode(buffer)? as u64),
                FormatCode::SMALL_INT => Primitive::Int(u8::decode(buffer)? as i32),
                FormatCode::SMALL_LONG => Primitive::Long(u8::decode(buffer)? as i64),
                FormatCode::USHORT => Primitive::UShort(Decode::decode(buffer)?),
                FormatCode::SHORT => Primitive::Short(Decode::decode(buffer)?),
                FormatCode::UINT => Primitive::UInt(Decode::decode(buffer)?),
                FormatCode::INT => Primitive::Int(Decode::decode(buffer)?),
                FormatCode::ULONG => Primitive::ULong(Decode::decode(buffer)?),
                FormatCode::LONG => Primitive::Long(Decode::decode(buffer)?),
                FormatCode::FLOAT => Primitive::Float(Decode::decode(buffer)?),
                FormatCode::DOUBLE => Primitive::Double(Decode::decode(buffer)?),
                FormatCode::DECIMAL32 => Primitive::Decimal32(),
                FormatCode::DECIMAL64 => Primitive::Decimal64(),
                FormatCode::DECIMAL128 => Primitive::Decimal128(),
                FormatCode::CHAR => Primitive::Char(Decode::decode(buffer)?),
                FormatCode::TIMESTAMP => {
                    todo!("timestamp is not implemented")
                }
                FormatCode::UUID => Primitive::Uuid(Decode::decode(buffer)?),
                FormatCode::BINARY8 => Primitive::Binary(variable_width(size1)(buffer)?.into()),
                FormatCode::BINARY32 => Primitive::Binary(variable_width(size4)(buffer)?.into()),
                FormatCode::STRING8_UTF8 => {
                    Primitive::String(String::from_utf8_lossy(variable_width(size1)(buffer)?))
                }
                FormatCode::STRING32_UTF8 => {
                    Primitive::String(String::from_utf8_lossy(variable_width(size4)(buffer)?))
                }
                FormatCode::SYMBOL8 => {
                    Primitive::Symbol(Symbol::new(variable_width(size1)(buffer)?))
                }
                FormatCode::SYMBOL32 => {
                    Primitive::Symbol(Symbol::new(variable_width(size4)(buffer)?))
                }
                FormatCode::LIST8 => Primitive::List(list(size1)(buffer)?),
                FormatCode::LIST32 => Primitive::List(list(size4)(buffer)?),
                FormatCode::MAP8 => Primitive::Map(map(size1)(buffer)?),
                FormatCode::MAP32 => Primitive::Map(map(size4)(buffer)?),
                FormatCode::ARRAY8 => Primitive::Array(array_primitive(size1)(buffer)?),
                FormatCode::ARRAY32 => Primitive::Array(array_primitive(size4)(buffer)?),
                FormatCode::Primitive(p) => {
                    return Err(io::Error::other(format!(
                        "Invalid primitive code 0x{p:02X}"
                    )))
                }
                FormatCode::Ext(_, _) => {
                    todo!("support ext format code")
                }
            }
            .into(),
            Constructor::Described {
                descriptor,
                constructor,
            } => {
                let value = constructor.construct_slice(buffer)?;
                Value::Described(Box::new(Described {
                    descriptor: descriptor.clone(),
                    value,
                }))
            }
        };
        Ok(value)
    }
}

fn variable_width<'a>(
    size: impl Fn(&mut &'a [u8]) -> io::Result<usize>,
) -> impl Fn(&mut &'a [u8]) -> io::Result<&'a [u8]> {
    move |buffer| {
        let n = size(buffer)?;
        if buffer.len() < n {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "unexpected eof",
            ));
        }
        let (data, rest) = buffer.split_at(n);
        *buffer = rest;
        Ok(data)
    }
}

impl CompoundIter {
    pub fn next_slice<'a>(&mut self, buffer: &mut &'a [u8]) -> io::Result<Option<Value<'a>>> {
        if self.count == 0 {
            return Ok(None);
        }
        let value = Value::view(buffer)?;
        self.count -= 1;
        Ok(Some(value))
    }
}

fn compound<R: std::io::Read>(
    size: impl Fn(&mut R) -> io::Result<usize>,
) -> impl Fn(&mut R) -> io::Result<CompoundIter> {
    move |reader| {
        let _sz = size(reader)?;
        let count = size(reader)?;
        Ok(CompoundIter::new(count))
    }
}

fn list<'a>(
    size: impl Fn(&mut &'a [u8]) -> io::Result<usize>,
) -> impl Fn(&mut &'a [u8]) -> io::Result<Vec<Value<'a>>> {
    let compound = compound(size);
    move |reader| {
        let mut iter = compound(reader)?;
        let mut list = Vec::with_capacity(iter.count);
        while let Some(value) = iter.next_slice(reader)? {
            list.push(value);
        }
        Ok(list)
    }
}

fn map<'a>(
    size: impl Fn(&mut &'a [u8]) -> io::Result<usize>,
) -> impl Fn(&mut &'a [u8]) -> io::Result<Vec<(Value<'a>, Value<'a>)>> {
    let compound = compound(size);
    move |reader| {
        let mut iter = compound(reader)?;
        let mut map = Vec::with_capacity(iter.count);
        while let (Some(key), Some(value)) = (iter.next(reader)?, iter.next(reader)?) {
            map.push((key, value));
        }
        Ok(map)
    }
}

impl<'a> ArrayIter<'a> {
    pub fn next_slice(&mut self, buffer: &mut &'a [u8]) -> io::Result<Option<Value<'a>>> {
        if self.count == 0 {
            return Ok(None);
        }
        let value = self.constructor.construct_slice(buffer)?;
        self.count -= 1;
        Ok(Some(value))
    }
}

fn array<'a>(
    size: impl Fn(&mut &'a [u8]) -> io::Result<usize>,
) -> impl Fn(&mut &'a [u8]) -> io::Result<ArrayIter<'a>> {
    move |reader| {
        let _sz = size(reader)?;
        dbg!(_sz);
        let count = size(reader)?;
        dbg!(count);
        let constructor = Constructor::view(reader)?;
        Ok(ArrayIter::new(count, constructor))
    }
}

fn array_primitive<'a>(
    size: impl Fn(&mut &'a [u8]) -> io::Result<usize>,
) -> impl Fn(&mut &'a [u8]) -> io::Result<Vec<Value<'a>>> {
    let array = array(size);
    move |reader| {
        let mut iter = array(reader)?;
        let mut list = Vec::with_capacity(iter.count);
        while let Some(value) = iter.next_slice(reader)? {
            list.push(value);
        }
        Ok(list)
    }
}
