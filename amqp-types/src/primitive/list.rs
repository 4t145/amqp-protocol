use std::io;

use crate::{codec::*, constructor::Constructor, format_code::FormatCode, value::Value};
#[derive(Debug, Clone, Default)]
pub struct ListIter<'frame> {
    pub(crate) count: usize,
    pub(crate) items_data: &'frame [u8],
}

impl<'frame> ListIter<'frame> {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> io::Result<Value<'frame>> {
        self.count -= 1;
        let constructor = Constructor::decode(&mut self.items_data)?;
        let size = constructor.format_code.peek_size(self.items_data)?;
        let item_data = self.items_data.try_eat(size)?;
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
            self.count -= 1;
            Some(unsafe { self.next_unchecked() })
        }
    }
}

pub struct List<'a> {
    inner: ListInner<'a>,
}

pub enum ListInner<'a> {
    Read(ListIter<'a>),
    Write(&'a mut dyn Iterator<Item = Value<'a>>),
}
