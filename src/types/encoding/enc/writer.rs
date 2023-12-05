use std::{
    io::{self, BufWriter, Write},
    slice,
};
use crate::types::{
    codes::FormatCode,
    value::{Descriptor, Primitive, Symbol, Value},
};

use crate::types::value::Constructor;

pub enum EncodeError {
    Io,
}

pub type EncodeResult<T> = Result<T, EncodeError>;

pub trait Encode<W: io::Write> {
    fn encode(&self, writer: &mut W) -> io::Result<()>;
}

impl<W: io::Write> Encode<W> for FormatCode {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        match self {
            FormatCode::Primitive(p) => p.encode(writer),
            FormatCode::Ext(p, ext) => {
                p.encode(writer)?;
                ext.encode(writer)
            }
        }
    }
}

impl<W: io::Write> Encode<W> for Symbol<'_> {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        self.bytes.as_ref().encode(writer)
    }
}

fn encode_variable<const S1: u8, const S4: u8, W: io::Write>(
    writer: &mut W,
    size: usize,
    value: &impl Encode<W>,
) -> io::Result<()> {
    if size < (u8::MAX as usize) {
        S1.encode(writer)?;
        (size as u8).encode(writer)?;
    } else {
        S4.encode(writer)?;
        (size as u32).encode(writer)?;
    }
    value.encode(writer)
}

fn encode_compound<const S1: u8, const S4: u8, W: io::Write>(
    writer: &mut W,
    size: usize,
    count: usize,
    value: &impl Encode<W>,
) -> io::Result<()> {
    if size + 4 < (u8::MAX as usize) && count < (u8::MAX as usize) {
        S1.encode(writer)?;
        let size = size + 1;
        (size as u8).encode(writer)?;
        (count as u8).encode(writer)?;
    } else {
        S4.encode(writer)?;
        let size = size + 4;
        (size as u32).encode(writer)?;
        (count as u32).encode(writer)?;
    }
    value.encode(writer)
}

impl<W: io::Write> Encode<W> for Descriptor<'_> {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Descriptor::Symbol(symbol) => encode_variable::<
                { FormatCode::SYMBOL8.into_u8() },
                { FormatCode::SYMBOL32.into_u8() },
                W,
            >(writer, symbol.bytes.len(), symbol),
            Descriptor::Numeric(d, id) => {
                FormatCode::ULONG.encode(writer)?;
                d.encode(writer)?;
                id.encode(writer)
            }
            Descriptor::Reserved(v) => v.encode(writer),
        }
    }
}

impl<W: io::Write> Encode<W> for Constructor<'_> {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Constructor::FormatCode(f) => f.encode(writer),
            Constructor::Described {
                descriptor,
                constructor,
            } => {
                0x00u8.encode(writer)?;
                descriptor.encode(writer)?;
                constructor.encode(writer)
            }
        }
    }
}

impl<W: io::Write> Encode<W> for Value<'_> {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        todo!()
    }
}

impl<W: io::Write> Encode<W> for &[u8] {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self)
    }
}

impl<W: io::Write> Encode<W> for u8 {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(slice::from_ref(self))
    }
}

impl<W: io::Write> Encode<W> for i8 {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(slice::from_ref(&(*self as u8)))
    }
}

impl<W: io::Write> Encode<W> for char {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        (*self as u32).encode(writer)
    }
}

impl<W: io::Write> Encode<W> for Primitive<'_> {
    fn encode(&self, writer: &mut W) -> io::Result<()> {
        match self {
            Primitive::Null => FormatCode::NULL.encode(writer),
            Primitive::Boolean(b) => match b {
                true => FormatCode::BOOLEAN_TRUE.encode(writer),
                false => FormatCode::BOOLEAN_FALSE.encode(writer),
            },
            Primitive::UByte(ub) => {
                FormatCode::UBYTE.encode(writer)?;
                ub.encode(writer)
            }
            Primitive::UShort(us) => {
                FormatCode::USHORT.encode(writer)?;
                us.encode(writer)
            }
            Primitive::UInt(ui) => {
                FormatCode::UINT.encode(writer)?;
                ui.encode(writer)
            }
            Primitive::ULong(ul) => {
                FormatCode::ULONG.encode(writer)?;
                ul.encode(writer)
            }
            Primitive::Byte(b) => {
                FormatCode::BYTE.encode(writer)?;
                b.encode(writer)
            }
            Primitive::Short(s) => {
                FormatCode::SHORT.encode(writer)?;
                s.encode(writer)
            }
            Primitive::Int(i) => {
                FormatCode::INT.encode(writer)?;
                i.encode(writer)
            }
            Primitive::Long(l) => {
                FormatCode::LONG.encode(writer)?;
                l.encode(writer)
            }
            Primitive::Float(f) => {
                FormatCode::FLOAT.encode(writer)?;
                f.encode(writer)
            }
            Primitive::Double(d) => {
                FormatCode::DOUBLE.encode(writer)?;
                d.encode(writer)
            }
            Primitive::Decimal32() => todo!(),
            Primitive::Decimal64() => todo!(),
            Primitive::Decimal128() => todo!(),
            Primitive::Char(c) => {
                FormatCode::CHAR.encode(writer)?;
                c.encode(writer)
            }
            Primitive::Timestamp(t) => {
                FormatCode::TIMESTAMP.encode(writer)?;
                t.encode(writer)
            }
            Primitive::Uuid(u) => {
                FormatCode::UUID.encode(writer)?;
                u.as_slice().encode(writer)
            }
            Primitive::String(s) => encode_variable::<
                { FormatCode::STRING8_UTF8.into_u8() },
                { FormatCode::STRING32_UTF8.into_u8() },
                _,
            >(writer, s.as_bytes().len(), &s.as_bytes()),
            Primitive::Binary(b) => encode_variable::<
                { FormatCode::BINARY8.into_u8() },
                { FormatCode::BINARY32.into_u8() },
                _,
            >(writer, b.len(), &b.as_ref()),
            Primitive::Symbol(s) => encode_variable::<
                { FormatCode::SYMBOL8.into_u8() },
                { FormatCode::SYMBOL32.into_u8() },
                _,
            >(writer, s.bytes.len(), &s.bytes.as_ref()),
            Primitive::List(l) => {
                let count = l.len();
                if count == 0 {
                    return FormatCode::LIST0.encode(writer);
                }
                let mut cursor = std::io::Cursor::new(Vec::with_capacity(l.len() * 2));
                for v in l {
                    v.encode(&mut cursor)?;
                }
                let size = cursor.position() as usize;
                let value = cursor.into_inner();
                encode_compound::<
                    { FormatCode::LIST8.into_u8() },
                    { FormatCode::LIST32.into_u8() },
                    _,
                >(writer, size, count, &value.as_slice())
            }
            Primitive::Map(m) => {
                let count = m.len() * 2;
                let mut cursor = std::io::Cursor::new(Vec::with_capacity(count * 4));
                for (k, v) in m {
                    k.encode(&mut cursor)?;
                    v.encode(&mut cursor)?;
                }
                let size = cursor.position() as usize;
                let value = cursor.into_inner();
                encode_compound::<{ FormatCode::MAP8.into_u8() }, { FormatCode::MAP32.into_u8() }, _>(
                    writer,
                    size,
                    count,
                    &value.as_slice(),
                )
            }
            Primitive::Array(a) => {
                todo!("encode array")
            }
        }
    }
}

macro_rules! rust_primitive {
    (be_number: $($type: ident)*) => {
        $(
            impl<W: io::Write> Encode<W> for $type {
                fn encode(&self, reader: &mut W) -> io::Result<()> {
                    reader.write_all(&<$type>::to_be_bytes(*self))
                }
            }
        )*
    };
}

rust_primitive! {
    be_number: u16 u32 u64 u128 i16 i32 i64 i128 f32 f64
}