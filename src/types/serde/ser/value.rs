use std::{borrow::Cow, marker::PhantomData};

use serde::ser;

use crate::types::value::{Primitive, Value, Binary};

use super::SerializeError;
#[derive(Default, Debug)]
pub struct Serializer<'a> {
    _phantom: PhantomData<Value<'a>>,
}

#[derive(Debug, Default)]
pub struct SerializeArray<'a> {
    list: Vec<Value<'a>>,
}

#[derive(Debug, Default)]
pub struct SerializeList<'a> {
    list: Vec<Value<'a>>,
}

impl SerializeList<'_> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            list: Vec::with_capacity(capacity),
        }
    }
}

impl<'a> ser::SerializeSeq for SerializeList<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.list.push(value.serialize(Serializer::default())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::List(self.list)))
    }
}

impl<'a> ser::SerializeTuple for SerializeList<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.list.push(value.serialize(Serializer::default())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::List(self.list)))
    }
}

impl<'a> ser::SerializeTupleStruct for SerializeList<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.list.push(value.serialize(Serializer::default())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::List(self.list)))
    }
}

impl<'a> ser::SerializeTupleVariant for SerializeList<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.list.push(value.serialize(Serializer::default())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::List(self.list)))
    }
}

#[derive(Debug, Default)]
pub struct SerializeMap<'a> {
    map: Vec<(Value<'a>, Value<'a>)>,
    key: Option<Value<'a>>,
}

impl SerializeMap<'_> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: Vec::with_capacity(capacity),
            key: None,
        }
    }
}

impl<'a> ser::SerializeMap for SerializeMap<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.key.replace(key.serialize(Serializer::default())?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(Serializer::default())?;
        let key = self.key.take().expect("missing key");
        self.map.push((key, value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Map(self.map)))
    }
}

impl<'a> ser::SerializeStruct for SerializeList<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.list.push(value.serialize(Serializer::default())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::List(self.list)))
    }
}

impl<'a> ser::SerializeStructVariant for SerializeList<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        self.list.push(value.serialize(Serializer::default())?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::List(self.list)))
    }
}

impl<'a> ser::Serializer for Serializer<'a> {
    type Ok = Value<'a>;

    type Error = SerializeError;

    type SerializeSeq = SerializeList<'a>;

    type SerializeTuple = SerializeList<'a>;

    type SerializeTupleStruct = SerializeList<'a>;

    type SerializeTupleVariant = SerializeList<'a>;

    type SerializeMap = SerializeMap<'a>;

    type SerializeStruct = SerializeList<'a>;

    type SerializeStructVariant = SerializeList<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Boolean(v)))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Byte(v)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Short(v)))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Int(v)))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Long(v)))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::UByte(v)))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::UShort(v)))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::UInt(v)))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::ULong(v)))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Float(v)))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Double(v)))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Char(v)))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::String(Cow::Owned(
            v.to_string(),
        ))))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Binary(Binary(Cow::Owned(v.to_vec())))))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::Primitive(Primitive::Null))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(len.map(SerializeList::with_capacity).unwrap_or_default())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SerializeList::with_capacity(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(SerializeList::with_capacity(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(SerializeList::with_capacity(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(len.map(SerializeMap::with_capacity).unwrap_or_default())
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerializeList::with_capacity(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerializeList::with_capacity(len))
    }
}
