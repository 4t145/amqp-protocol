use crate::{constructor::Constructor, value::Value, io::WriteAble};
#[derive(Debug, Clone)]
pub struct ListIter<'frame> {
    count: usize,
    datas: &'frame [u8]
}

impl<'frame> Iterator for ListIter<'frame> {
    type Item = Value<'frame>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub enum List<'a, T> {
    Read(ListIter<'a>),
    Write(&'a mut dyn Iterator<Item = T>) 
}

impl<'a, T> List<'a, T> {
    pub fn write( iter: &'a mut impl Iterator<Item = T>) -> Self {
        Self::Write(iter)
    }
}
impl<'a, T: WriteAble> Into<Value<'a>> for List<'a, T> {
    fn into(self) -> Value<'a> {
        match self {
            List::Read(r) => {
                todo!()
            },
            List::Write(w) => {
                todo!()
            },
        }
        
    }
}