pub use array::*;
pub use binary::*;
pub use list::*;
pub use map::*;
pub use symbol::*;
pub use ts::*;
pub use uuid::*;

mod array;
mod binary;
mod list;
mod map;
mod symbol;
mod ts;
mod uuid;

#[derive(Debug, Clone)]
pub enum Primitive<'frame> {
    Null,
    Boolean(bool),
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Decimal32(),
    Decimal64(),
    Decimal128(),
    Char(char),
    Timestamp(Ts),
    Uuid(Uuid),
    String(&'frame str),
    Binary(Binary<'frame>),
    Symbol(Symbol<'frame>),
    List(ListIter<'frame>),
    Map(MapIter<'frame>),
    Array(ArrayIter<'frame>),
}

macro_rules! derive_from_and_try_into {
    ($($id:ident: $Type: ty)*) => {
        $(
            impl From<$Type> for Primitive<'_> {
                fn from(value: $Type) -> Self {
                    Primitive::$id(value)
                }
            }
            impl TryInto<$Type> for Primitive<'_> {
                type Error = ();
                fn try_into(self) -> Result<$Type, Self::Error> {
                    match self {
                        Primitive::$id(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
    ($($id:ident: $Type: ident as $lt:lifetime )*) => {
        $(
            impl<$lt> From<$Type<$lt>> for Primitive<$lt> {
                fn from(value: $Type<$lt>) -> Self {
                    Primitive::$id(value)
                }
            }
            impl<$lt> TryInto<$Type<$lt>> for Primitive<$lt> {
                type Error = ();
                fn try_into(self) -> Result<$Type<$lt>, Self::Error> {
                    match self {
                        Primitive::$id(v) => Ok(v),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}

derive_from_and_try_into! {
    // Null: ()
    Boolean: bool
    UByte: u8
    UShort: u16
    UInt: u32
    ULong: u64
    Byte: i8
    Short: i16
    Int: i32
    Long: i64
    Float: f32
    Double: f64
    Timestamp: Ts
    // Decimal32: ()
    // Decimal64: ()
    // Decimal128: ()
    Char: char
    Uuid: Uuid

}

derive_from_and_try_into! {
    Binary: Binary as 'a
    Symbol: Symbol as 'a
    List: ListIter as 'a
    Map: MapIter as 'a
    Array: ArrayIter as 'a
}

impl<'a> From<&'a str> for Primitive<'a> {
    fn from(value: &'a str) -> Self {
        Primitive::String(value)
    }
}

impl<'a> TryInto<&'a str> for Primitive<'a> {
    type Error = ();
    fn try_into(self) -> Result<&'a str, Self::Error> {
        match self {
            Primitive::String(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl<'a, T> From<Option<T>> for Primitive<'a>
where
    Primitive<'a>: From<T>,
{
    fn from(value: Option<T>) -> Self {
        match value {
            Some(x) => Primitive::from(x),
            None => Primitive::Null,
        }
    }
}

// impl<'a, T> TryInto<Option<T>> for Primitive<'a>
// where
//     Primitive<'a>: TryInto<Option<T>>,
// {
//     type Error = ();
//     fn try_into(self) -> Result<&'a str, Self::Error> {
//         match self {
//             Primitive::Null => Ok(None),
//             _ => Primitive::try_into(self).map(Some),
//         }
//     }
// }
