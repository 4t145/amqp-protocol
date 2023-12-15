use crate::error::UNEXPECTED_TYPE;
use crate::codec::Encode;
use crate::try_take_n;
use crate::types::{Type, Multiple};
use crate::{constructor::Constructor, value::Value};
use std::io;

use super::Primitive;
#[derive(Debug, Clone)]
pub struct ArrayIter<'frame> {
    pub constructor: Constructor<'frame>,
    pub count: usize,
    pub items_data: &'frame [u8],
}

impl<'frame> ArrayIter<'frame> {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> io::Result<Value<'frame>> {
        self.count -= 1;
        let size = self.constructor.peek_size(self.items_data)?;
        let item_data = try_take_n(&mut self.items_data, size)?;
        self.items_data = &self.items_data[..size];
        Ok(Value::new(self.constructor.clone(), item_data))
    }
}

impl<'frame> Iterator for ArrayIter<'frame> {
    type Item = io::Result<Value<'frame>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            Some(unsafe { self.next_unchecked() })
        }
    }
}

#[repr(transparent)]
pub struct Array<'a, T> {
    pub iter: ArrayInner<'a, T>,
}

impl<'a, T: Multiple<'a>> Array<'a, T> {
    pub fn new(iter: impl IntoIterator<Item = T>) -> Self {
        Self {
            iter: ArrayInner::Data(ArrayIter {
                constructor: T::CONSTRUCTOR,
                count: iter.into_iter().count(),
                items_data: &[],
            }),
        }
    }
}

impl<'a, T> From<ArrayIter<'a>> for Array<'a, T> {
    fn from(value: ArrayIter<'a>) -> Self {
        Self {
            iter: ArrayInner::Data(value),
        }
    }
}

pub enum ArrayInner<'a, T> {
    Data(ArrayIter<'a>),
    Iter(&'a mut dyn Iterator<Item = T>),
}

impl<'a, T: Type<'a>> Iterator for Array<'a, T> {
    type Item = io::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            ArrayInner::Data(d) => d.next().map(|v| v.and_then(TryInto::try_into)),
            ArrayInner::Iter(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a, T: Type<'a>> TryFrom<Value<'a>> for Array<'a, T> {
    type Error = io::Error;
    fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
        let Primitive::Array(iter) = value.construct()? else {
            return Err(io::Error::other(UNEXPECTED_TYPE));
        };
        Ok(iter.into())
    }
}


impl<'a, T: Type<'a>> Encode for Array<'a, T> {
    fn encode(self, buffer: &mut [u8]) -> io::Result<()> {
        for item in self {
            item?.encode(buffer)?
        }
        Ok(())
    }
}