use serde::de::Error as _;
use serde::de::{self, IntoDeserializer};
use std::{borrow::Cow, fmt::Display, io, vec::IntoIter};

use crate::{
    primitives::{AmqpArray, AmqpList, AmqpMap, Primitive},
    value::Value,
};

#[derive(Debug)]
pub struct DeserializeError(io::Error);

impl DeserializeError {
    pub fn into_inner(self) -> io::Error {
        self.0
    }
}

impl std::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for DeserializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl serde::de::Error for DeserializeError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        io::Error::other(msg.to_string()).into()
    }
}

impl From<io::Error> for DeserializeError {
    fn from(err: io::Error) -> Self {
        DeserializeError(err)
    }
}

impl From<DeserializeError> for io::Error {
    fn from(val: DeserializeError) -> Self {
        val.0
    }
}

impl<'de> de::Deserializer<'de> for Primitive {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self {
            Primitive::Null => visitor.visit_none(),
            Primitive::Boolean(b) => visitor.visit_bool(b),
            Primitive::UByte(b) => visitor.visit_u8(b),
            Primitive::UShort(s) => visitor.visit_u16(s),
            Primitive::UInt(i) => visitor.visit_u32(i),
            Primitive::ULong(l) => visitor.visit_u64(l),
            Primitive::Byte(b) => visitor.visit_i8(b),
            Primitive::Short(s) => visitor.visit_i16(s),
            Primitive::Int(i) => visitor.visit_i32(i),
            Primitive::Long(l) => visitor.visit_i64(l),
            Primitive::Float(f) => visitor.visit_f32(f),
            Primitive::Double(d) => visitor.visit_f64(d),
            Primitive::Decimal32() => todo!(),
            Primitive::Decimal64() => todo!(),
            Primitive::Decimal128() => todo!(),
            Primitive::Char(c) => visitor.visit_char(c),
            Primitive::Timestamp(t) => visitor.visit_u64(t),
            Primitive::Uuid(u) => visitor.visit_bytes(&u),
            Primitive::String(s) => {
                // we are not zero copy here
                visitor.visit_string(s.as_str().map_err(Self::Error::custom)?.to_string())
            }
            Primitive::Binary(b) => visitor.visit_bytes(b.0.as_ref()),
            Primitive::Symbol(s) => visitor.visit_bytes(s.0.as_ref()),
            Primitive::List(l) => {
                let value = visitor.visit_seq(l)?;
                Ok(value)
            }
            Primitive::Map(m) => {
                let value = visitor.visit_map(MapAccess::new(m))?;
                Ok(value)
            }
            Primitive::Array(a) => {
                let value = visitor.visit_seq(a)?;
                Ok(value)
            }
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
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
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }
}

impl<'de> de::IntoDeserializer<'de, DeserializeError> for Primitive {
    type Deserializer = Self;

    fn into_deserializer(self) -> Self::Deserializer {
        self
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
    list: IntoIter<&'a Primitive>,
}

impl<'a> ListAccess<'a> {
    pub fn new(list: Vec<&'a Primitive>) -> ListAccess {
        ListAccess {
            list: list.into_iter(),
        }
    }
    pub fn empty() -> ListAccess<'a> {
        ListAccess {
            list: Vec::new().into_iter(),
        }
    }
}
impl<'de, 'a: 'de> de::SeqAccess<'de> for AmqpList {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        use serde::de::Error;
        if let Some(value) = self.next() {
            Ok(Some(seed.deserialize(value.construct().ok_or(
                Self::Error::custom("fail to construct primitive value"),
            )?)?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.count)
    }
}

impl<'de, 'a: 'de> de::SeqAccess<'de> for AmqpArray {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.next() {
            Ok(Some(seed.deserialize(value)?))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.count)
    }
}

pub struct MapAccess {
    map: AmqpMap,
    value: Option<Primitive>,
}
impl MapAccess {
    pub fn new(map: AmqpMap) -> Self {
        Self { map, value: None }
    }
}

impl<'de, 'a: 'de> de::MapAccess<'de> for MapAccess {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        use serde::de::Error;

        if let Some((key, val)) = self.map.next() {
            self.value.replace(
                val.construct()
                    .ok_or(Self::Error::custom("fail to construct primitive value"))?,
            );
            Ok(Some(seed.deserialize(val.construct().ok_or(
                Self::Error::custom("fail to construct primitive value"),
            )?)?))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        if let Some(val) = self.value.take() {
            seed.deserialize(val.into_deserializer())
        } else {
            unreachable!("should always hava a value")
        }
    }
}
