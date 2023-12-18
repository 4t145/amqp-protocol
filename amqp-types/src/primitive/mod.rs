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

impl<'a> Primitive<'a> {
    pub fn is_null(&self) -> bool {
        matches!(self, Primitive::Null)
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Primitive::Boolean(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Primitive::Byte(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Primitive::UByte(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Primitive::Short(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Primitive::UShort(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Primitive::Int(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Primitive::UInt(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Primitive::Long(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Primitive::ULong(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Primitive::Float(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Primitive::Double(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_char(&self) -> Option<char> {
        match self {
            Primitive::Char(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_ts(&self) -> Option<Ts> {
        match self {
            Primitive::Timestamp(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_uuid(&self) -> Option<Uuid> {
        match self {
            Primitive::Uuid(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            Primitive::String(v) => Some(*v),
            _ => None,
        }
    }
    pub fn as_binary(&self) -> Option<Binary<'a>> {
        match self {
            Primitive::Binary(v) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn as_symbol(&self) -> Option<Symbol<'a>> {
        match self {
            Primitive::Symbol(v) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<ListIter<'a>> {
        match self {
            Primitive::List(v) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn as_map(&self) -> Option<MapIter<'a>> {
        match self {
            Primitive::Map(v) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn as_array(&self) -> Option<ArrayIter<'a>> {
        match self {
            Primitive::Array(v) => Some(v.clone()),
            _ => None,
        }
    }
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
