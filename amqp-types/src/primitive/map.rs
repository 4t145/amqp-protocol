use std::io;

use crate::{constructor::Constructor, error::UNEXPECTED_TYPE, types::Type, value::Value};

use super::{ListIter, Primitive};
#[derive(Debug, Clone)]
pub struct MapIter<'frame> {
    inner: ListIter<'frame>,
}

impl<'frame> Iterator for MapIter<'frame> {
    type Item = io::Result<(Value<'frame>, Value<'frame>)>;
    fn next(&mut self) -> Option<Self::Item> {
        let key = self.inner.next()?;
        match key {
            Ok(key) => {
                let value = self.inner.next()?;
                match value {
                    Ok(value) => Some(Ok((key, value))),
                    Err(err) => Some(Err(err)),
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'a> From<ListIter<'a>> for MapIter<'a> {
    fn from(value: ListIter<'a>) -> Self {
        Self { inner: value }
    }
}

pub struct Map<'a, K, V> {
    inner: MapInner<'a, K, V>,
}

impl<'a, K, V> From<MapIter<'a>> for Map<'a, K, V> {
    fn from(value: MapIter<'a>) -> Self {
        Self {
            inner: MapInner::Read(value),
        }
    }
}

pub enum MapInner<'a, K, V> {
    Read(MapIter<'a>),
    Write(&'a mut dyn Iterator<Item = (K, V)>),
}

impl<'a, K: Type<'a>, V: Type<'a>> Map<'a, K, V> {
    pub fn new(iter: &'a mut impl Iterator<Item = (K, V)>) -> Self {
        Self {
            inner: MapInner::Write(iter as &mut dyn Iterator<Item = (K, V)>),
        }
    }
}

impl<'a, K: Type<'a>, V: Type<'a>> Iterator for Map<'a, K, V> {
    type Item = io::Result<(K, V)>;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            MapInner::Read(iter) => iter.next().map(|res| match res {
                Ok((k, v)) => {
                    let k = K::try_from_value(k)?;
                    let v = V::try_from_value(v)?;
                    Ok((k, v))
                }
                Err(e) => Err(e),
            }),
            MapInner::Write(iter) => {
                let (key, value) = iter.next()?;
                Some(Ok((key, value)))
            }
        }
    }
}

impl<'a, K: Type<'a>, V: Type<'a>> TryFrom<Value<'a>> for Map<'a, K, V> {
    type Error = io::Error;
    fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
        let Primitive::Map(iter) = value.construct()? else {
            return Err(io::Error::other(UNEXPECTED_TYPE));
        };
        Ok(iter.into())
    }
}
