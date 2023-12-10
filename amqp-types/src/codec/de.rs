use crate::{
    codes::FormatCode,
    constructor::Constructor,
    data::{Data, DataView},
    descriptor::Descriptor,
    value::Value,
};
use bytes::Bytes;
pub trait BytesExt: Sized {
    fn try_eat(&mut self, n: usize) -> Option<Self>;
    fn try_eat_n<const N: usize>(&mut self) -> Option<[u8; N]>;
    fn peek_n<const N: usize>(&self) -> Option<[u8; N]>;
}

impl BytesExt for Bytes {
    #[inline]
    fn try_eat(&mut self, n: usize) -> Option<Self> {
        debug_assert!(self.len() >= n, "try_eat_n: {} >= {}", self.len(), n);
        (self.len() >= n).then(|| self.split_to(n))
    }
    #[inline]
    fn peek_n<const N: usize>(&self) -> Option<[u8; N]> {
        debug_assert!(self.len() >= N, "try_peek_n: {} >= {}", self.len(), N);
        let bytes = (self.len() >= N).then(|| &self[0..N])?;
        <[u8; N]>::try_from(bytes).ok()
    }
    #[inline]
    fn try_eat_n<const N: usize>(&mut self) -> Option<[u8; N]> {
        debug_assert!(self.len() >= N, "try_eat_n: {} >= {}", self.len(), N);
        let bytes = (self.len() >= N).then(|| self.split_to(N))?;
        <[u8; N]>::try_from(bytes.as_ref()).ok()
    }
}

pub trait Decode: Sized {
    fn decode(bytes: &mut Bytes) -> Option<Self>;
}

impl Decode for Value {
    fn decode(data: &mut Bytes) -> Option<Self> {
        let constructor = Constructor::decode(data)?;
        let size = constructor.format_code.peek_size(data)?;
        let data = data.try_eat(size)?;
        Some(Value { constructor, data })
    }
}

impl Decode for Constructor {
    fn decode(bytes: &mut Bytes) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }
        // 1. peek first byte
        let mut first_byte = u8::decode(bytes)?;
        let mut descriptor = Option::None;
        // 2. if first byte is 0x00, it should be a descriptor
        if first_byte == 0x00 {
            descriptor.replace(Descriptor::decode(bytes)?);
            first_byte = u8::decode(bytes)?;
        }
        debug_assert!(first_byte != 0x00, "don't support double described type");

        let format_code = if first_byte & 0x0f != 0x0f {
            FormatCode::Primitive(first_byte)
        } else {
            if bytes.is_empty() {
                return None;
            }
            let ext_byte = u8::decode(bytes)?;
            FormatCode::Ext(first_byte, ext_byte)
        };

        Some(Constructor {
            format_code,
            descriptor,
        })
    }
}

impl Decode for Descriptor {
    fn decode(bytes: &mut Bytes) -> Option<Self> {
        let value = Value::decode(bytes)?;
        let primitive = value.construct()?;
        Some(match primitive {
            crate::primitives::Primitive::ULong(code) => Descriptor::Numeric(code),
            crate::primitives::Primitive::Symbol(s) => Descriptor::Symbol(s),
            _ => Descriptor::Reserved(Box::new(value)),
        })
    }
}

macro_rules! derive_primitives {
    (@numbers $($pt: ty)*) => {
        $(
            impl Decode for $pt {
                fn decode(data: &mut Bytes) -> Option<Self> {
                    let bytes = data.try_eat_n::<{ std::mem::size_of::<$pt>() }>()?;
                    Some(<$pt>::from_be_bytes(bytes))
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

impl Decode for bool {
    fn decode(bytes: &mut Bytes) -> Option<Self> {
        u8::decode(bytes).map(|b| b != 0)
    }
}

impl<const N: usize> Decode for [u8; N] {
    fn decode(data: &mut Bytes) -> Option<Self> {
        data.try_eat_n::<N>()
    }
}
