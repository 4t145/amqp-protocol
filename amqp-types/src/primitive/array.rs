use crate::codec::Encode;
use crate::error::UNEXPECTED_TYPE;
use crate::types::{Multiple, Type};
use crate::{constructor::Constructor, value::Value};
use std::fmt::Debug;
use std::io;

use super::Primitive;
#[derive(Debug, Clone, Default)]
pub struct ArrayIter<'frame> {
    pub constructor: Constructor<'frame>,
    pub count: usize,
    pub items_data: &'frame [u8],
}

fn try_take_n<'b>(bytes: &mut &'b [u8], size: usize) -> io::Result<&'b [u8]> {
    if bytes.len() > size {
        Err(io::Error::other("invalid amqp data: no enough bytes"))
    } else {
        let (take, rest) = bytes.split_at(size);
        *bytes = rest;
        Ok(take)
    }
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

impl<'a, T> Default for Array<'a, T> {
    fn default() -> Self {
        Self {
            iter: ArrayInner::Data(ArrayIter {
                constructor: Constructor::default(),
                count: 0,
                items_data: &[],
            }),
        }
    }
}

impl<'a, T> std::fmt::Debug for Array<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Array").finish()
    }
}

impl<'a, T: Multiple> Array<'a, T> {
    pub fn new_write(iter: (impl IntoIterator<Item = T> + 'a)) -> Self {
        Self {
            iter: ArrayInner::Iter(Box::new(iter.into_iter())),
        }
    }
    pub fn new_read(iter: ArrayIter<'a>) -> Self {
        Self {
            iter: ArrayInner::Data(iter),
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
    Iter(Box<(dyn Iterator<Item = T> + 'a)>),
}

impl<'a, T: Type<'a>> Iterator for Array<'a, T> {
    type Item = io::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            ArrayInner::Data(d) => d.next().map(|v| v.and_then(T::try_from_value)),
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

// impl<'a, T: Type<'a>> Encode for Array<'a, T> {
//     fn encode(self, buffer: &mut [u8]) -> io::Result<()> {
//         for item in self {
//             item?.encode(buffer)?
//         }
//         Ok(())
//     }
// }
