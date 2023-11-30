use std::{fmt::Display, io};

use serde::de;

use super::*;

pub struct Deserializer<R> {
    reader: R,
}

impl<R: io::Read> Deserializer<R> {
    pub fn eat_type<T: Decode<'byte>>(&mut self) -> DecodeResult<T> {
        self.reader.take(n)
        self.eat::<T, _>(T::decode)
    }
    pub fn eat_constructor(&mut self) -> DecodeResult<()> {
        self.constructor = self.eat_type::<Constructor>()?;
        Ok(())
    }
    pub fn eat<T, F>(&mut self, f: F) -> DecodeResult<T>
    where
        F: Fn(&'byte [u8]) -> (DecodeResult<T>, &'byte [u8]),
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

pub struct ListAccess<'a> {
    count: usize,
    de: &'a mut Deserializer<'a>,
}

impl<'a,> ListAccess<'a> {
    pub fn with_count(count: usize, de: &'a mut Deserializer<'a>) -> ListAccess<'a> {
        ListAccess { count, de }
    }
    pub fn empty(de: &'a mut Deserializer<'a>) -> ListAccess<'a> {
        ListAccess { count: 0, de }
    }
}

impl<'de, 'a: 'de> de::SeqAccess<'de> for ListAccess<'a> {
    type Error = DecodeErrorKind;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.count == 0 {
            Ok(None)
        } else {
            // read constructor
            self.de.eat_constructor()?;
            self.count -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.count)
    }
}
impl<'de: 'byte, 'byte> de::Deserializer<'de> for &'de mut Deserializer<'byte> {
    type Error = DecodeErrorKind;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        // let constructor = self.take_type::<Constructor>()?;
        match &self.constructor {
            Constructor::FormatCode(code) => match *code {
                FormatCode::NULL => visitor.visit_none(),
                FormatCode::BOOLEAN_TRUE => visitor.visit_bool(true),
                FormatCode::BOOLEAN_FALSE => visitor.visit_bool(false),
                FormatCode::UINT_0 => visitor.visit_u32(0),
                FormatCode::ULONG_0 => visitor.visit_u64(0),
                FormatCode::LIST0 => visitor.visit_seq(ListAccess::empty(self)),
                FormatCode::BOOLEAN => visitor.visit_bool(0 == self.eat(u8("boolean"))?),
                FormatCode::UBYTE => visitor.visit_u8(self.eat(u8("ubyte"))?),
                FormatCode::BYTE => visitor.visit_i8(self.eat(u8("byte"))? as i8),
                FormatCode::SMALL_UINT => visitor.visit_u32(self.eat(u8("uint"))? as u32),
                FormatCode::SMALL_ULONG => visitor.visit_u64(self.eat(u8("ulong"))? as u64),
                FormatCode::SMALL_INT => visitor.visit_i32(self.eat(u8("int"))? as i32),
                FormatCode::SMALL_LONG => visitor.visit_i64(self.eat(u8("long"))? as i64),
                FormatCode::USHORT => visitor.visit_u16(self.eat(u16("ushort"))?),
                FormatCode::SHORT => visitor.visit_i16(self.eat(i16("short"))?),
                FormatCode::UINT => visitor.visit_i32(self.eat(i32("uint"))?),
                FormatCode::INT => visitor.visit_i32(self.eat(i32("int"))?),
                FormatCode::ULONG => visitor.visit_u64(self.eat(u64("ulong"))?),
                FormatCode::LONG => visitor.visit_i64(self.eat(i64("long"))?),
                FormatCode::FLOAT => visitor.visit_f32(self.eat(f32("float"))?),
                FormatCode::DOUBLE => visitor.visit_f64(self.eat(f64("double"))?),
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
                    let charcode = self.eat(u32("char"))?;
                    let char =
                        char::from_u32(charcode).ok_or(DecodeErrorKind::InvalidChar(charcode))?;
                    visitor.visit_char(char)
                }
                FormatCode::TIMESTAMP => {
                    todo!()
                }
                FormatCode::UUID => {
                    let uuid = self.eat(n_bytes(16, "uuid"))?;
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
        match self.constructor {
            Constructor::FormatCode(_) => todo!(),
            Constructor::Described {
                descriptor,
                constructor,
            } => todo!(),
        }
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
        // visitor.visit_newtype_struct(deserializer)
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
