use std::fmt::Display;

use serde::de;

use super::*;

pub struct Deserializer<'de> {
    constructor: Constructor<'de>,
    bytes: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn take_type<T: Decode<'de>>(&mut self) -> DecodeResult<T> {
        self.take::<T, _>(T::try_decode)
    }
    pub fn take<T, F>(&mut self, f: F) -> DecodeResult<T>
    where
        F: Fn(&'de [u8]) -> (DecodeResult<T>, &'de [u8]),
    {
        let (value, bytes) = f(self.bytes);
        self.bytes = bytes;
        value
    }
    pub fn size(&self) -> DecodeResult<usize> {
        self.constructor.size(self.bytes)
    }
}

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Error {}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

pub struct ListAccess<'de> {
    count: usize,
    data: &'de [u8],
}

impl<'de> ListAccess<'de> {
    pub fn with_count(count: usize, data: &'de [u8]) -> ListAccess<'de> {
        ListAccess { count, data }
    }
    pub fn empty() -> ListAccess<'de> {
        ListAccess {
            count: 0,
            data: &[],
        }
    }
}

impl<'de> de::SeqAccess<'de> for ListAccess<'de> {
    type Error = DecodeErrorKind;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.count == 0 {
            Ok(None)
        } else {
            // read constructor
            let (constructor, bytes) = Constructor::try_decode(self.data);
            let constructor = constructor?;
            self.data = bytes;
            let size = constructor.size(bytes)?;

            let (item_bytes, rest) = n_bytes(size, "n bytes data")(bytes);
            self.data = rest;
            let value = seed.deserialize(Deserializer {
                constructor,
                bytes: item_bytes?,
            })?;
            self.count -= 1;
            Ok(Some(value))
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.count)
    }
}
impl<'de> de::Deserializer<'de> for Deserializer<'de> {
    type Error = DecodeErrorKind;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let constructor = self.take_type::<Constructor>()?;
        match constructor {
            Constructor::FormatCode(code) => match code {
                FormatCode::NULL => visitor.visit_none(),
                FormatCode::BOOLEAN_TRUE => visitor.visit_bool(true),
                FormatCode::BOOLEAN_FALSE => visitor.visit_bool(false),
                FormatCode::UINT_0 => visitor.visit_u32(0),
                FormatCode::ULONG_0 => visitor.visit_u64(0),
                FormatCode::LIST0 => visitor.visit_seq(ListAccess::empty()),
                FormatCode::BOOLEAN => visitor.visit_bool(0 == self.take(u8("boolean"))?),
                FormatCode::UBYTE => visitor.visit_u8(self.take(u8("ubyte"))?),
                FormatCode::BYTE => visitor.visit_i8(self.take(u8("byte"))? as i8),
                FormatCode::SMALL_UINT => visitor.visit_u32(self.take(u8("uint"))? as u32),
                FormatCode::SMALL_ULONG => visitor.visit_u64(self.take(u8("ulong"))? as u64),
                FormatCode::SMALL_INT => visitor.visit_i32(self.take(u8("int"))? as i32),
                FormatCode::SMALL_LONG => visitor.visit_i64(self.take(u8("long"))? as i64),
                FormatCode::USHORT => {
                    todo!()
                }
                FormatCode::SHORT => {
                    todo!()
                }
                FormatCode::UINT => {
                    todo!()
                }
                FormatCode::INT => {
                    todo!()
                }
                FormatCode::ULONG => {
                    todo!()
                }
                FormatCode::LONG => {
                    todo!()
                }
                FormatCode::FLOAT => {
                    todo!()
                }
                FormatCode::DOUBLE => {
                    todo!()
                }
                FormatCode::DECIMAL32 => {
                    todo!()
                }
                FormatCode::DECIMAL64 => {
                    todo!()
                }
                FormatCode::DECIMAL128 => {
                    todo!()
                }
                FormatCode::CHAR => {
                    todo!()
                }
                FormatCode::TIMESTAMP => {
                    todo!()
                }
                FormatCode::UUID => {
                    let uuid = self.take(n_bytes(16, "uuid"))?;
                    visitor.visit_bytes(uuid)
                }
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
            },
            Constructor::Described {
                descriptor,
                constructor,
            } => todo!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
}
