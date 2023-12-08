use crate::{
    codes::FormatCode,
    constructor::Constructor,
    data::{Data, DataView},
    value::Value,
};
use bytes::Bytes;
pub trait BytesExt: Sized {
    fn try_eat(&mut self, n: usize) -> Option<Self>;
    fn try_eat_n<const N: usize>(&mut self) -> Option<[u8; N]>;
}

impl BytesExt for Bytes {
    #[inline]

    fn try_eat(&mut self, n: usize) -> Option<Self> {
        (self.len() >= n).then_some(self.split_to(n))
    }

    fn try_eat_n<const N: usize>(&mut self) -> Option<[u8; N]> {
        let bytes = (self.len() >= N).then_some(self.split_to(N))?;
        <[u8; N]>::try_from(bytes.as_ref()).ok()
    }
}
pub trait Decode: Sized {
    fn decode(bytes: &mut Bytes) -> Option<Self>;
}

impl Decode for Value {
    fn decode(data: &mut Bytes) -> Option<Self> {
        let constructor = Constructor::decode(data)?;
        let size = constructor.format_code.take_size(data)?;
        let data = data.try_eat(size)?;
        let data = Data(data);
        Some(Value { constructor, data })
    }
}

impl Decode for Constructor {
    fn decode(bytes: &mut Bytes) -> Option<Self> {
        if bytes.is_empty() {
            return None;
        }
        // 1. peek first byte
        let mut first_byte = bytes.split_to(1)[0];
        // 2. if first byte is 0x00, it should be a descriptor
        if first_byte == 0x00 {
            let descriptor = Value::decode(bytes)?;
            if bytes.is_empty() {
                return None;
            }
            first_byte = bytes.split_to(1)[0];
        }
        debug_assert!(first_byte != 0x00, "don't support double described type");
        if first_byte & 0x0f != 0x0f {
            Some(Self::new(FormatCode::Primitive(first_byte)))
        } else {
            if bytes.is_empty() {
                return None;
            }
            let ext_byte = bytes.split_to(1)[0];
            Some(Constructor::new(FormatCode::Ext(first_byte, ext_byte)))
        }
    }
}
