use std::{io, ops::DerefMut};

use crate::{codec::*, constructor::Constructor, value::Value, Data};
#[derive(Debug, Clone, Default)]
pub struct ListIter<'frame> {
    pub(crate) count: usize,
    pub(crate) items_data: Data<'frame>,
}

impl<'frame> ListIter<'frame> {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> io::Result<Value<'frame>> {
        self.count -= 1;
        let data = self.items_data.deref_mut();
        let constructor = Constructor::decode(data)?;
        let size = constructor.format_code.peek_size(data)?;
        let item_data = data.try_eat(size)?;
        let value = Value::new(constructor, item_data);
        Ok(value)
    }
}

impl<'frame> Iterator for ListIter<'frame> {
    type Item = io::Result<Value<'frame>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            Some(unsafe { self.next_unchecked() })
        }
    }
}

// pub struct List<'a> {
//     inner: ListInner<'a>,
// }

// pub enum ListInner<'a> {
//     Read(ListIter<'a>),
//     Write(&'a mut dyn Iterator<Item = Value<'a>>),
// }
