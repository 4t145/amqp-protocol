use crate::io::WriteAble;

#[derive(Debug, Clone)]
pub struct Data<'a> {
    inner: DataInner<'a>,
}

#[derive(Debug, Clone)]
enum DataInner<'a> {
    Read(&'a [u8]),
    Write(&'a dyn WriteAble),
}


impl<'a> Data<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            inner: DataInner::Read(bytes),
        }
    }
    pub const fn new_write(bytes: &'a impl WriteAble) -> Self {
        Self {
            inner: DataInner::Write(bytes),
        }
    }
    pub fn as_write(&mut self) {
        
    }
}

impl<'a> From<&'a [u8]> for Data<'a> {
    fn from(val: &'a [u8]) -> Self {
        Self::new(val)
    }
}
