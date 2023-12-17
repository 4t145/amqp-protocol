use std::io;

use crate::{
    constructor::Constructor,
    descriptor::Descriptor,
    error::*,
    format_code::FormatCode,
    primitive::{Primitive, Uuid},
    value::Value,
};
pub trait DecodeExt: Sized {
    fn split_to(&mut self, n: usize) -> Self;
    fn try_eat(&mut self, n: usize) -> io::Result<Self>;
    fn try_eat_n<const N: usize>(&mut self) -> io::Result<[u8; N]>;
    fn peek_n<const N: usize>(&self) -> io::Result<[u8; N]>;
    fn peek(&self, n: usize) -> io::Result<&[u8]>;
    /// # Safety
    /// call if after `try_eat` or `try_eat_n`, after parsing failed
    unsafe fn backward(&mut self, n: usize);
}

impl DecodeExt for &[u8] {
    fn split_to(&mut self, n: usize) -> Self {
        let (take, rest) = self.split_at(n);
        *self = take;
        rest
    }
    #[inline]
    fn try_eat(&mut self, n: usize) -> io::Result<Self> {
        debug_assert!(self.len() >= n, "try_eat_n: {} >= {}", self.len(), n);
        if self.len() < n {
            return Err(io::Error::other(BUFFER_SIZE_ERROR));
        }
        Ok(self.split_to(n))
    }
    #[inline]
    fn peek(&self, n: usize) -> io::Result<&[u8]> {
        debug_assert!(self.len() >= n, "try_peek_n: {} >= {}", self.len(), n);
        if self.len() < n {
            return Err(io::Error::other(BUFFER_SIZE_ERROR));
        }
        Ok(&self[0..n])
    }
    #[inline]
    fn peek_n<const N: usize>(&self) -> io::Result<[u8; N]> {
        debug_assert!(self.len() >= N, "try_peek_n: {} >= {}", self.len(), N);
        if self.len() < N {
            return Err(io::Error::other(BUFFER_SIZE_ERROR));
        }
        <[u8; N]>::try_from(&self[0..N]).map_err(io::Error::other)
    }
    #[inline]
    fn try_eat_n<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        debug_assert!(self.len() >= N, "try_eat_n: {} >= {}", self.len(), N);
        if self.len() < N {
            return Err(io::Error::other(BUFFER_SIZE_ERROR));
        }
        <[u8; N]>::try_from(self.split_to(N)).map_err(io::Error::other)
    }
    unsafe fn backward(&mut self, n: usize) {
        let ptr = self.as_ptr().sub(n);
        *self = std::slice::from_raw_parts(ptr, self.len() + n);
    }
}

pub trait Decode<'de>: Sized {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self>;
}

impl<'de> Decode<'de> for Value<'de> {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        let constructor = Constructor::decode(data)?;
        let size = constructor.format_code.peek_size(data)?;
        let data = data.try_eat(size)?;
        Ok(Value::new(constructor, data))
    }
}

impl<'de> Decode<'de> for Constructor<'de> {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        if data.is_empty() {
            return Err(io::Error::other(BUFFER_SIZE_ERROR));
        }
        // 1. peek first byte
        let mut first_byte = u8::decode(data)?;
        let mut descriptor = None;
        // 2. if first byte is 0x00, it should be a descriptor
        if first_byte == 0x00 {
            descriptor.replace(Descriptor::decode(data)?);
            first_byte = u8::decode(data)?;
        }
        debug_assert!(first_byte != 0x00, "don't support double described type");

        let format_code = if first_byte & 0x0f != 0x0f {
            FormatCode::Primitive(first_byte)
        } else {
            if data.is_empty() {
                return Err(io::Error::other(BUFFER_SIZE_ERROR));
            }
            let ext_byte = u8::decode(data)?;
            FormatCode::Ext(first_byte, ext_byte)
        };

        Ok(Constructor {
            format_code,
            descriptor,
        })
    }
}

impl<'de> Decode<'de> for Descriptor<'de> {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        let value = Value::decode(data)?;
        let primitive = value.construct()?;
        Ok(match primitive {
            Primitive::ULong(code) => Descriptor::Numeric(code),
            Primitive::Symbol(s) => Descriptor::Symbol(s),
            _ => Descriptor::Reserved(),
        })
    }
}

macro_rules! derive_primitives {
    (@numbers $($pt: ty)*) => {
        $(
            impl<'de> Decode<'de> for $pt {
                fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
                    let data = data.try_eat_n::<{ std::mem::size_of::<$pt>() }>()?;
                    Ok(<$pt>::from_be_bytes(data))
                }
            }
        )*
    };
}

derive_primitives! {
    @numbers
    i8 i16 i32 i64 i128
    u8 u16 u32 u64 u128
    f32 f64
}

impl<'de> Decode<'de> for bool {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        u8::decode(data).map(|b| b != 0)
    }
}

impl<'de> Decode<'de> for char {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        let code_point = data.peek_n::<4>()?;
        match char::from_u32(u32::from_be_bytes(code_point)) {
            Some(c) => {
                *data = data.split_to(4);
                Ok(c)
            }
            None => Err(io::Error::other(INVALID_UTF32_CODE_POINT)),
        }
    }
}

impl<'de, const N: usize> Decode<'de> for [u8; N] {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        data.try_eat_n::<N>()
    }
}

impl<'de> Decode<'de> for Uuid {
    fn decode(data: &mut &'de [u8]) -> io::Result<Self> {
        let uuid = data.try_eat_n::<16>()?;
        Ok(uuid.into())
    }
}

pub fn decode_str<'de>(data: &mut &'de [u8], size: usize) -> io::Result<&'de str> {
    let str_bytes = data.try_eat(size)?;
    match std::str::from_utf8(str_bytes) {
        Ok(s) => Ok(s),
        Err(e) => {
            unsafe { data.backward(size) };
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid utf8 string: {}", e),
            ))
        }
    }
}
