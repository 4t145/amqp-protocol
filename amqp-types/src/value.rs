use std::io;

use crate::error::UNEXPECTED_TYPE;
use crate::primitive::*;
use crate::{constructor::Constructor, data::Data, primitive::Primitive};
#[derive(Debug, Clone, Default)]
pub struct Value<'frame> {
    pub constructor: Constructor<'frame>,
    pub data: Data<'frame>,
}

impl<'frame> Value<'frame> {
    pub fn new(constructor: impl Into<Constructor<'frame>>, data: impl Into<Data<'frame>>) -> Self {
        Value {
            constructor: constructor.into(),
            data: data.into(),
        }
    }
    pub fn construct(self) -> io::Result<Primitive<'frame>> {
        let Self { constructor, data } = self;
        constructor.construct(data)
    }
}

macro_rules! derive_try_from {
    ($($id:ident: $Type: ty)*) => {
        $(
            impl TryFrom<Value<'_>> for $Type {
                type Error = io::Error;
                fn try_from(value: Value<'_>) -> Result<$Type, Self::Error> {
                    value.construct()?.try_into().map_err(|_|io::Error::other(UNEXPECTED_TYPE))
                }
            }
        )*
    };
    ({$lt:lifetime} $($id:ident: $Type: ident )*) => {
        $(
            impl<$lt> TryFrom<Value<$lt>> for $Type<$lt> {
                type Error = io::Error;
                fn try_from(value: Value<$lt>) -> Result<$Type<$lt>, Self::Error> {
                    value.construct()?.try_into().map_err(|_|io::Error::other(UNEXPECTED_TYPE))
                }
            }
        )*
    };
}

// derive_try_from! {
//     // Null: ()
//     Boolean: bool
//     UByte: u8
//     UShort: u16
//     UInt: u32
//     ULong: u64
//     Byte: i8
//     Short: i16
//     Int: i32
//     Long: i64
//     Float: f32
//     Double: f64
//     // Decimal32: ()
//     // Decimal64: ()
//     // Decimal128: ()
//     Char: char
//     Uuid: Uuid
//     Timestamp: Ts
// }

// derive_try_from! {
//     {'a}
//     Binary: Binary
//     Symbol: Symbol
//     List: ListIter
//     Map: MapIter
//     Array: ArrayIter
// }

// impl<'a> TryFrom<Value<'a>> for &'a str {
//     type Error = io::Error;

//     fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
//         value
//             .construct()?
//             .try_into()
//             .map_err(|_| io::Error::other(UNEXPECTED_TYPE))
//     }
// }
