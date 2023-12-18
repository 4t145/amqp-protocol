use crate::{codec::Encode, constructor::Constructor, error, primitive::*, value::Value};
use std::io;

mod restrict;
pub use restrict::Restrict;
mod multiple;
pub use multiple::Multiple;

pub trait Type<'a>: Encode + Restrict {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error>;
}

impl<'a> Type<'a> for i8 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_i8()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for i16 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_i16()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for i32 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_i32()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for i64 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_i64()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for u8 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_u8()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for u16 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_u16()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for u32 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_u32()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for u64 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_u64()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for f32 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_f32()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for f64 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_f64()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for char {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_char()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for Uuid {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_uuid()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for Ts {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_ts()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for &'a str {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_str()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for Symbol<'a> {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_symbol()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a> Type<'a> for Binary<'a> {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        value
            .construct()?
            .as_binary()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))
    }
}

impl<'a, T: Type<'a> + Multiple> Type<'a> for Array<'a, T> {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        let primitive = value.construct()?;
        let array = primitive
            .as_array()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))?;
        Ok(Array::new_read(array))
    }
}

impl<'a, K: Type<'a>, V: Type<'a>> Type<'a> for Map<'a, K, V> {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        let primitive = value.construct()?;
        let map = primitive
            .as_map()
            .ok_or(io::Error::other(error::UNEXPECTED_TYPE))?;
        Ok(Map::new_read(map))
    }
}

impl<'a, T: Type<'a>> Type<'a> for Option<T> {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        let primitive = value.clone().construct()?;
        if primitive.is_null() {
            Ok(None)
        } else {
            Ok(Some(T::try_from_value(value)?))
        }
    }
}
