use std::{fmt, io, mem::size_of, slice::Iter, string};

use crate::types::codes::FormatCode;

use crate::types::value::{Binary, Constructor, Described, Descriptor, Primitive, Symbol, Value};

use super::{ArrayIter, CompoundIter};

impl Constructor<'static> {
    pub fn construct<R: io::Read>(&self, reader: &mut R) -> io::Result<Value<'static>> {
        // let constructor = self.take_type::<Constructor>()?;
        let value = match self {
            Constructor::FormatCode(code) => match *code {
                FormatCode::NULL => Primitive::Null,
                FormatCode::BOOLEAN_TRUE => Primitive::Boolean(true),
                FormatCode::BOOLEAN_FALSE => Primitive::Boolean(false),
                FormatCode::UINT_0 => Primitive::UInt(0),
                FormatCode::ULONG_0 => Primitive::ULong(0),
                FormatCode::LIST0 => Primitive::List(vec![]),
                FormatCode::BOOLEAN => Primitive::Boolean(0 == u8::decode(reader)?),
                FormatCode::UBYTE => Primitive::UByte(u8::decode(reader)?),
                FormatCode::BYTE => Primitive::Byte(i8::decode(reader)?),
                FormatCode::SMALL_UINT => Primitive::UInt(u8::decode(reader)? as u32),
                FormatCode::SMALL_ULONG => Primitive::ULong(u8::decode(reader)? as u64),
                FormatCode::SMALL_INT => Primitive::Int(u8::decode(reader)? as i32),
                FormatCode::SMALL_LONG => Primitive::Long(u8::decode(reader)? as i64),
                FormatCode::USHORT => Primitive::UShort(Decode::decode(reader)?),
                FormatCode::SHORT => Primitive::Short(Decode::decode(reader)?),
                FormatCode::UINT => Primitive::UInt(Decode::decode(reader)?),
                FormatCode::INT => Primitive::Int(Decode::decode(reader)?),
                FormatCode::ULONG => Primitive::ULong(Decode::decode(reader)?),
                FormatCode::LONG => Primitive::Long(Decode::decode(reader)?),
                FormatCode::FLOAT => Primitive::Float(Decode::decode(reader)?),
                FormatCode::DOUBLE => Primitive::Double(Decode::decode(reader)?),
                FormatCode::DECIMAL32 => Primitive::Decimal32(),
                FormatCode::DECIMAL64 => Primitive::Decimal64(),
                FormatCode::DECIMAL128 => Primitive::Decimal128(),
                FormatCode::CHAR => Primitive::Char(Decode::decode(reader)?),
                FormatCode::TIMESTAMP => {
                    todo!("timestamp is not implemented")
                }
                FormatCode::UUID => Primitive::Uuid(Decode::decode(reader)?),
                FormatCode::BINARY8 => {
                    Primitive::Binary(Binary(variable_width(size1)(reader)?.into()))
                }
                FormatCode::BINARY32 => {
                    Primitive::Binary(Binary(variable_width(size4)(reader)?.into()))
                }
                FormatCode::STRING8_UTF8 => Primitive::String(
                    String::from_utf8(variable_width(size1)(reader)?)
                        .map_err(io::Error::other)?
                        .into(),
                ),
                FormatCode::STRING32_UTF8 => Primitive::String(
                    String::from_utf8(variable_width(size4)(reader)?)
                        .map_err(io::Error::other)?
                        .into(),
                ),
                FormatCode::SYMBOL8 => {
                    Primitive::Symbol(Symbol::new(variable_width(size1)(reader)?))
                }
                FormatCode::SYMBOL32 => {
                    Primitive::Symbol(Symbol::new(variable_width(size4)(reader)?))
                }
                FormatCode::LIST8 => Primitive::List(list(size1)(reader)?),
                FormatCode::LIST32 => Primitive::List(list(size4)(reader)?),
                FormatCode::MAP8 => Primitive::Map(map(size1)(reader)?),
                FormatCode::MAP32 => Primitive::Map(map(size4)(reader)?),
                FormatCode::ARRAY8 => Primitive::Array(array_primitive(size1)(reader)?),
                FormatCode::ARRAY32 => Primitive::Array(array_primitive(size4)(reader)?),
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
                let value = constructor.construct(reader)?;
                Value::Described(Box::new(Described {
                    descriptor: descriptor.clone(),
                    value,
                }))
            }
        };
        Ok(value)
    }
}

pub trait Decode<R: io::Read>: Sized {
    fn decode(reader: &mut R) -> io::Result<Self>;
}

pub fn bytes<R: std::io::Read>(n: usize) -> impl Fn(&mut R) -> io::Result<Vec<u8>> {
    move |reader| {
        let mut buf = vec![0; n];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

impl<const N: usize, R: std::io::Read> Decode<R> for [u8; N] {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let mut buf = [0; N];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

macro_rules! rust_primitive {
    (be_number: $($type: ident)*) => {
        $(
            impl<R: std::io::Read> Decode<R> for $type {
                fn decode(reader: &mut R) -> io::Result<Self> {
                    let mut buf = [0;size_of::<$type>()];
                    reader.read_exact(&mut buf)?;
                    let n = <$type>::from_be_bytes(buf);
                    Ok(n)
                }
            }
        )*
    };
}

rust_primitive! {
    be_number: u16 u32 u64 u128 i16 i32 i64 i128 f32 f64
}

impl<R: std::io::Read> Decode<R> for u8 {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let mut byte = 0u8;
        reader.read_exact(std::slice::from_mut(&mut byte))?;
        Ok(byte)
    }
}

impl<R: std::io::Read> Decode<R> for i8 {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let mut byte = 0u8;
        reader.read_exact(std::slice::from_mut(&mut byte))?;
        Ok(byte as i8)
    }
}

impl<R: std::io::Read> Decode<R> for char {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let ch = u32::decode(reader)?;
        char::from_u32(ch).ok_or(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid charcode u{ch:04x}"),
        ))
    }
}

fn variable_width<R: std::io::Read>(
    size: impl Fn(&mut R) -> io::Result<usize>,
) -> impl Fn(&mut R) -> io::Result<Vec<u8>> {
    move |reader| {
        let n = size(reader)?;
        let mut buf = vec![0; n];
        reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

pub fn size1<R: io::Read>(reader: &mut R) -> io::Result<usize> {
    u8::decode(reader).map(|n| n as usize)
}

pub fn size4<R: io::Read>(reader: &mut R) -> io::Result<usize> {
    u32::decode(reader).map(|n| n as usize)
}

impl CompoundIter {
    pub fn next<R: io::Read>(&mut self, reader: &mut R) -> io::Result<Option<Value<'static>>> {
        if self.count > 0 {
            let value = Value::decode(reader)?;
            self.count -= 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
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

fn list<R: io::Read>(
    size: impl Fn(&mut R) -> io::Result<usize>,
) -> impl Fn(&mut R) -> io::Result<Vec<Value<'static>>> {
    let compound = compound(size);
    move |reader| {
        let mut iter = compound(reader)?;
        let mut list = Vec::with_capacity(iter.count);
        while let Some(value) = iter.next(reader)? {
            list.push(value);
        }
        Ok(list)
    }
}

fn map<R: io::Read>(
    size: impl Fn(&mut R) -> io::Result<usize>,
) -> impl Fn(&mut R) -> io::Result<Vec<(Value<'static>, Value<'static>)>> {
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

impl ArrayIter<'static> {
    pub fn next<R: io::Read>(&mut self, reader: &mut R) -> io::Result<Option<Value<'static>>> {
        if self.count > 0 {
            let value = self.constructor.construct(reader)?;
            self.count -= 1;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

fn array<R: std::io::Read>(
    size: impl Fn(&mut R) -> io::Result<usize>,
) -> impl Fn(&mut R) -> io::Result<ArrayIter<'static>> {
    move |reader| {
        let _sz = size(reader)?;
        let count = size(reader)?;
        let constructor = Constructor::decode(reader)?;
        Ok(ArrayIter::new(count, constructor))
    }
}

fn array_primitive<R: std::io::Read>(
    size: impl Fn(&mut R) -> io::Result<usize>,
) -> impl Fn(&mut R) -> io::Result<Vec<Value<'static>>> {
    let array = array(size);
    move |reader| {
        let mut iter = array(reader)?;
        let mut list = Vec::with_capacity(iter.count);
        while let Some(value) = iter.next(reader)? {
            list.push(value);
        }
        Ok(list)
    }
}

impl<R: io::Read> Decode<R> for Constructor<'static> {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let byte = u8::decode(reader)?;
        match byte {
            0x00 => {
                let descriptor = Descriptor::decode(reader)?;
                let constructor = Constructor::decode(reader)?;
                Ok(Constructor::Described {
                    descriptor,
                    constructor: Box::new(constructor),
                })
            }
            code if code & 0x0f != 0x0f => Ok(Constructor::FormatCode(FormatCode::Primitive(code))),
            code => {
                let ext = u8::decode(reader)?;
                Ok(Constructor::FormatCode(FormatCode::Ext(code, ext)))
            }
        }
    }
}

impl<R: io::Read> Decode<R> for Value<'static> {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let constructor = Constructor::decode(reader)?;
        constructor.construct(reader)
    }
}

impl<R: io::Read> Decode<R> for Descriptor<'static> {
    fn decode(reader: &mut R) -> io::Result<Self> {
        let constructor = Constructor::decode(reader)?;
        let descriptor = constructor.construct(reader)?;
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

impl Value<'static> {
    #[inline]
    pub fn from_reader<R: io::Read>(reader: &mut R) -> io::Result<Self> {
        Self::decode(reader)
    }
}
