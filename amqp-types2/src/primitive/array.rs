use crate::try_take_n;
use crate::{constructor::Constructor, value::Value};
use std::io;
#[derive(Debug, Clone)]
pub struct ArrayIter<'frame> {
    pub constructor: Constructor<'frame>,
    pub count: usize,
    pub datas: &'frame [u8],
}

impl<'frame> ArrayIter<'frame> {
    #[inline]
    unsafe fn next_unchecked(&mut self) -> io::Result<Value<'frame>> {
        self.count -= 1;
        let size = self.constructor.size_hint(self.datas.into())?;
        let item_data = try_take_n(&mut self.datas, size)?;
        self.datas = &self.datas[..size];
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

#[derive(Debug, Clone)]
pub struct Array<'frame, T> {
    pub iter: ArrayIter<'frame>,
    pub _marker: std::marker::PhantomData<T>,
}


