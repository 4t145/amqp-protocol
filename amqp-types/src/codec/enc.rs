use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
};

use bytes::BufMut;

use crate::{
    constructor::Constructor,
    descriptor::Descriptor,
    error::UNEXPECTED_TYPE,
    format_code::FormatCode,
    primitive::{Array, Binary, Map, Symbol, Uuid, Ts},
    types::{Multiple, Type},
    value::Value,
};

pub trait Encode<'a>: Sized {
    const DESCRIPTOR: Option<Descriptor<'static>> = None;
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode;
    /// we should panic here if the format code is invalid since the format code is determained by ourselves.
    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()>;
    fn encode(self, constructor: Constructor, mut buffer: &mut [u8]) -> io::Result<()> {
        let format_code = constructor.format_code;
        write_constructor(buffer, constructor)?;
        self.encode_data(format_code, buffer)
    }
    fn encode_default(self, mut buffer: &mut [u8]) -> io::Result<()> {
        write_constructor(
            buffer,
            Constructor {
                descriptor: Self::DESCRIPTOR,
                format_code: Self::ENCODE_DEFAULT_FORMAT_CODE,
            },
        )?;
        self.encode_data(Self::ENCODE_DEFAULT_FORMAT_CODE, buffer)
    }
}

// impl<'a> Encode<'a> for &[u8] {
//     fn encode(self, mut buffer: &mut [u8]) -> io::Result<()> {
//         buffer.write_all(self)
//     }
// }

// impl<'x, T: WriteAble> for Array<T>
fn write_format_code(mut buffer: &mut [u8], format_code: FormatCode) -> io::Result<()> {
    match format_code {
        FormatCode::Primitive(p) => buffer.put_u8(p),
        FormatCode::Ext(c, e) => {
            buffer.put_u8(c);
            buffer.put_u8(e);
        }
    }
    Ok(())
}

fn write_descriptor(buffer: &mut [u8], descriptor: Descriptor) -> io::Result<()> {
    match descriptor {
        Descriptor::Symbol(s) => s.encode_default(buffer)?,
        Descriptor::Numeric(n) => n.encode_default(buffer)?,
        Descriptor::Reserved() => {
            unimplemented!();
        }
    };
    Ok(())
}

fn write_constructor(mut buffer: &mut [u8], constructor: Constructor) -> io::Result<()> {
    if let Some(descriptor) = constructor.descriptor {
        buffer.put_u8(0x00);
        write_descriptor(buffer, descriptor)?
    }
    write_format_code(buffer, constructor.format_code)
}

impl<'a> Encode<'a> for u8 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::UBYTE;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_u8(self);
        Ok(())
    }
}

impl<'a> Encode<'a> for i8 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::BYTE;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_i8(self);
        Ok(())
    }
}

impl<'a> Encode<'a> for u16 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::USHORT;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_u16(self);
        Ok(())
    }
}

impl<'a> Encode<'a> for i16 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::SHORT;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_i16(self);
        Ok(())
    }
}

impl<'a> Encode<'a> for u32 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::UINT;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::UINT => buffer.put_u32(self),
            FormatCode::UINT_0 => {
                debug_assert_eq!(self, 0)
            }
            FormatCode::SMALL_UINT => {
                debug_assert!(self <= u8::MAX as u32);
                buffer.put_u8(self as u8)
            }
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for i32 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::INT;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::INT => buffer.put_i32(self),
            FormatCode::SMALL_INT => {
                debug_assert!(i8::try_from(self).is_ok());
                buffer.put_i8(self as i8)
            }
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for u64 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::ULONG;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::ULONG => buffer.put_u64(self),
            FormatCode::ULONG_0 => {
                debug_assert_eq!(self, 0)
            }
            FormatCode::SMALL_ULONG => {
                debug_assert!(self <= u8::MAX as u64);
                buffer.put_u8(self as u8)
            }
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for i64 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::LONG;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::LONG => buffer.put_i64(self),
            FormatCode::SMALL_LONG => {
                debug_assert!(i8::try_from(self).is_ok());
                buffer.put_i8(self as i8)
            }
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for f32 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::FLOAT;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_f32(self);
        Ok(())
    }
}

impl<'a> Encode<'a> for f64 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::DOUBLE;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_f64(self);
        Ok(())
    }
}

impl<'a> Encode<'a> for bool {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::BOOLEAN;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::BOOLEAN => {
                buffer.put_u8(self as u8);
            }
            FormatCode::BOOLEAN_FALSE => {}
            FormatCode::BOOLEAN_TRUE => {}
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for char {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::CHAR;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_u32(self as u32);
        Ok(())
    }
}

impl<'a> Encode<'a> for Uuid {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::UUID;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_slice(&self.0);
        Ok(())
    }
}

impl<'a> Encode<'a> for Ts {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::TIMESTAMP;

    fn encode_data(self, format_code: FormatCode, mut buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        buffer.put_u64(self.0);
        Ok(())
    }
}

fn write_slice_8(slice: &[u8], mut buffer: &mut [u8]) {
    let size = slice.len();
    debug_assert!(u8::try_from(size).is_ok());
    let size_u8 = size as u8;
    buffer.put_u8(size_u8);
    buffer.put_slice(slice);
}

fn write_slice_32(slice: &[u8], mut buffer: &mut [u8]) {
    let size = slice.len();
    debug_assert!(u32::try_from(size).is_ok());
    let size_u32 = size as u32;
    buffer.put_u32(size_u32);
    buffer.put_slice(slice);
}

impl<'a> Encode<'a> for &'a str {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::SYMBOL32;

    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::STRING8_UTF8 => write_slice_8(self.as_bytes(), buffer),
            FormatCode::STRING32_UTF8 => write_slice_32(self.as_bytes(), buffer),
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}
impl<'a> Encode<'a> for Symbol<'a> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::SYMBOL32;

    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::SYMBOL8 => write_slice_8(self.0, buffer),
            FormatCode::SYMBOL32 => write_slice_32(self.0, buffer),
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for Binary<'a> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::BINARY32;

    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()> {
        match format_code {
            FormatCode::BINARY8 => write_slice_8(self.0, buffer),
            FormatCode::BINARY32 => write_slice_32(self.0, buffer),
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

pub fn write_items_8(
    mut buffer: &mut [u8],
    fwrite: impl FnOnce(&mut [u8]) -> io::Result<usize>,
) -> io::Result<()> {
    let size_ptr = buffer.as_mut_ptr();
    buffer.put_u8(0);
    let count_ptr = buffer.as_mut_ptr();
    buffer.put_u8(0);
    let count = fwrite(buffer)?;
    unsafe {
        let size = buffer.as_ptr().offset_from(count_ptr);
        debug_assert!(u8::try_from(size).is_ok());
        size_ptr.write(size as u8);
        debug_assert!(u8::try_from(count).is_ok());
        count_ptr.write(size as u8);
    }
    Ok(())
}

pub fn write_items_32(
    mut buffer: &mut [u8],
    fwrite: impl FnOnce(&mut [u8]) -> io::Result<usize>,
) -> io::Result<()> {
    let size_ptr = buffer.as_mut_ptr();
    buffer.put_u8(0);
    let count_ptr = buffer.as_mut_ptr();
    buffer.put_u8(0);
    let count = fwrite(buffer)?;
    unsafe {
        let size = buffer.as_ptr().offset_from(count_ptr);
        debug_assert!(u32::try_from(size).is_ok());
        let size = u32::to_be_bytes(size as u32);
        size_ptr.copy_from_nonoverlapping(size.as_ptr(), 4);
        debug_assert!(u32::try_from(count).is_ok());
        let count = u32::to_be_bytes(count as u32);
        count_ptr.copy_from_nonoverlapping(count.as_ptr(), 4);
    }
    Ok(())
}

impl<'a, T: Multiple + Type<'a>> Encode<'a> for Array<'a, T> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::ARRAY32;

    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()> {
        let fwrite = move |buffer: &mut [u8]| {
            let mut count: usize = 0;
            for item in self {
                let item = item?;
                item.encode_data(T::ENCODE_DEFAULT_FORMAT_CODE, buffer)?;
                count += 1;
            }
            io::Result::Ok(count)
        };
        match format_code {
            FormatCode::ARRAY8 => {
                write_items_8(buffer, fwrite)?;
            }
            FormatCode::ARRAY32 => {
                write_items_32(buffer, fwrite)?;
            }
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a, K: Type<'a>, V: Type<'a>> Encode<'a> for Map<'a, K, V> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::MAP32;

    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()> {
        let fwrite = move |buffer: &mut [u8]| {
            let mut count: usize = 0;
            for item in self {
                let (k, v) = item?;
                k.encode_default(buffer)?;
                v.encode_default(buffer)?;
                count += 2;
            }
            io::Result::Ok(count)
        };
        match format_code {
            FormatCode::MAP8 => {
                write_items_8(buffer, fwrite)?;
            }
            FormatCode::MAP32 => {
                write_items_32(buffer, fwrite)?;
            }
            _ => panic!("invalid format code"),
        }
        Ok(())
    }
}

impl<'a> Encode<'a> for () {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::NULL;

    fn encode_data(self, format_code: FormatCode, _buffer: &mut [u8]) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        Ok(())
    }
}

impl<'a, T: Type<'a>> Encode<'a> for Option<T> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = T::ENCODE_DEFAULT_FORMAT_CODE;
    const DESCRIPTOR: Option<Descriptor<'static>> = T::DESCRIPTOR;
    fn encode_data(self, format_code: FormatCode, buffer: &mut [u8]) -> io::Result<()> {
        match self {
            Some(v) => v.encode_data(format_code, buffer),
            None => ().encode_data(FormatCode::NULL, buffer),
        }
    }

    fn encode(self, constructor: Constructor, mut buffer: &mut [u8]) -> io::Result<()> {
        match self {
            Some(v) => v.encode(constructor, buffer),
            None => ().encode_default(buffer),
        }
    }

    fn encode_default(self, mut buffer: &mut [u8]) -> io::Result<()> {
        match self {
            Some(v) => v.encode_default(buffer),
            None => ().encode_default(buffer),
        }
    }
}
