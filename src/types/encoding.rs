// constructor = format-code
// / %x00 descriptor constructor
// format-code = fixed / variable / compound / array
// fixed = empty / fixed-one / fixed-two / fixed-four
// / fixed-eight / fixed-sixteen
// variable = variable-one / variable-four
// compound = compound-one / compound-four
// array = array-one / array-four
// descriptor = value
// value = constructor untyped-bytes
// untyped-bytes = *OCTET ; this is not actually *OCTET, the
// ; valid byte sequences are restricted
// ; by the constructor
// ; fixed width format codes
// empty = %x40-4E / %x4F %x00-FF
// fixed-one = %x50-5E / %x5F %x00-FF
// fixed-two = %x60-6E / %x6F %x00-FF
// fixed-four = %x70-7E / %x7F %x00-FF
// fixed-eight = %x80-8E / %x8F %x00-FF
// fixed-sixteen = %x90-9E / %x9F %x00-FF
// ; variable width format codes
// variable-one = %xA0-AE / %xAF %x00-FF
// variable-four = %xB0-BE / %xBF %x00-FF
// ; compound format codes
// compound-one = %xC0-CE / %xCF %x00-FF
// compound-four = %xD0-DE / %xDF %x00-FF
// ; array format codes
// array-one = %xE0-EE / %xEF %x00-FF
// array-four = %xF0-FE / %xFF %x00-FF

use std::{fmt, io, mem::size_of, slice::Iter};

use self::codes::FormatCode;

use super::value::{Primitive, Value};
mod codes;
// mod de;

#[derive(Debug)]
pub enum DecodeErrorKind {
    Expect(&'static str),
    InvalidFormatCode(&'static str, u8),
    InvalidChar(u32),
    Io(io::Error),
}

impl From<io::Error> for DecodeErrorKind {
    fn from(value: io::Error) -> Self {
        DecodeErrorKind::Io(value)
    }
}

impl serde::de::Error for DecodeErrorKind {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        todo!()
    }
}
impl std::error::Error for DecodeErrorKind {}
impl fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub type DecodeResult<T> = Result<T, DecodeErrorKind>;

pub enum Constructor {
    FormatCode(FormatCode),
    Described {
        descriptor: Box<Descriptor>,
        constructor: Box<Constructor>,
    },
}

impl Constructor {
    pub fn decode_value<R: io::Read>(self, reader: &mut R) -> DecodeResult<Value> {
        // let constructor = self.take_type::<Constructor>()?;
        let value = match self {
            Constructor::FormatCode(code) => match code {
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
                FormatCode::CHAR => {
                    let charcode = u32::decode(reader)?;
                    let ch =
                        char::from_u32(charcode).ok_or(DecodeErrorKind::InvalidChar(charcode))?;
                    Primitive::Char(ch)
                }
                FormatCode::TIMESTAMP => {
                    todo!()
                }
                FormatCode::UUID => Primitive::Uuid(Decode::decode(reader)?),
                FormatCode::BINARY8 => {
                    todo!()
                }
                FormatCode::BINARY32 => {
                    todo!()
                }
                FormatCode::STRING8_UTF8 => {
                    todo!()
                }
                FormatCode::STRING32_UTF8 => {
                    todo!()
                }
                FormatCode::SYMBOL8 => {
                    todo!()
                }
                FormatCode::SYMBOL32 => {
                    todo!()
                }
                FormatCode::LIST8 => {
                    todo!()
                }
                FormatCode::LIST32 => {
                    todo!()
                }
                FormatCode::MAP8 => {
                    todo!()
                }
                FormatCode::MAP32 => {
                    todo!()
                }
                FormatCode::ARRAY8 => {
                    todo!()
                }
                FormatCode::ARRAY32 => {
                    todo!()
                }
                FormatCode::Primitive(_) => todo!(),
                FormatCode::Ext(_, _) => {
                    todo!()
                }
            }
            .into(),
            Constructor::Described {
                descriptor,
                constructor,
            } => todo!(),
        };
        Ok(value)
    }
}

pub struct Descriptor {
    pub constructor: Constructor,
    pub value: Value,
}

pub trait Decode<R: io::Read>: Sized {
    fn decode(bytes: &mut R) -> DecodeResult<Self>;
}

// fn take_n<'a>(
//     bytes: &'a [u8],
//     n: usize,
//     expect: &'static str,
// ) -> (DecodeResult<&'a [u8]>, &'a [u8]) {
//     let (taked, rest) = bytes.split_at(n);
//     if taked.len() == n {
//         (Ok(taked), rest)
//     } else {
//         (Err(DecodeErrorKind::Expect(expect)), bytes)
//     }
// }

fn bytes<const N: usize, R: std::io::Read>(reader: &mut R) -> DecodeResult<[u8; N]> {
    let mut buf = [0; N];
    reader.read_exact(&mut buf);
    Ok(buf)
}

impl<const N: usize, R: std::io::Read> Decode<R> for [u8; N] {
    fn decode(reader: &mut R) -> DecodeResult<Self> {
        let mut buf = [0; N];
        reader.read_exact(&mut buf);
        Ok(buf)
    }
}

macro_rules! rust_primitive {
    (be_number: $($type: ident)*) => {
        $(
            impl<R: std::io::Read> Decode<R> for $type {
                fn decode(reader: &mut R) -> DecodeResult<Self> {
                    let mut buf = [0;size_of::<$type>()];
                    reader.read_exact(&mut buf);
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
    fn decode(reader: &mut R) -> DecodeResult<Self> {
        let mut byte = 0u8;
        reader.read_exact(std::slice::from_mut(&mut byte))?;
        Ok(byte)
    }
}

impl<R: std::io::Read> Decode<R> for i8 {
    fn decode(reader: &mut R) -> DecodeResult<Self> {
        let mut byte = 0u8;
        reader.read_exact(std::slice::from_mut(&mut byte))?;
        Ok(byte as i8)
    }
}

fn size1<R: io::Read>(reader: &mut R) -> DecodeResult<u8> {
    u8::decode(reader)
}

fn size4<R: io::Read>(reader: &mut R) -> DecodeResult<u32> {
    u32::decode(reader)
}

fn value() {}

macro_rules! tri {
    ($T: ty, $bytes: expr) => {{
        let (result, bytes) = <$T>::try_decode($bytes);
        match result {
            Ok(result) => (result, bytes),
            Err(err) => return (Err(err), bytes),
        }
    }};
}

impl<R: io::Read> Decode<R> for Constructor {
    fn decode(reader: &mut R) -> DecodeResult<Self> {
        let byte = u8::decode(reader)?;
        match byte {
            0x00 => {
                let descriptor = Descriptor::decode(reader)?;
                let constructor = Constructor::decode(reader)?;
                Ok(Constructor::Described {
                    descriptor: Box::new(descriptor),
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

impl<'a, R: io::Read + 'a> Decode<R> for Value {
    fn decode(reader: &mut R) -> DecodeResult<Self> {
        let constructor = Constructor::decode(reader)?;
        constructor.decode_value(reader)
    }
}

impl<R: io::Read> Decode<R> for Descriptor {
    fn decode(reader: &mut R) -> DecodeResult<Self> {
        let constructor = Constructor::decode(reader)?;
        let value = constructor.decode_value(reader)?;
        Ok(Descriptor { constructor, value })
    }
}
