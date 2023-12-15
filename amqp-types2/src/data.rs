use crate::codec::Encode;

#[derive(Debug, Clone)]
pub struct Data<'a> {
    inner: &'a [u8],
}

impl<'a> Data<'a> {
    pub fn into_inner(self) -> &'a [u8] {
        self.inner
    }
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { inner: bytes }
    }
}

impl<'a> From<&'a [u8]> for Data<'a> {
    fn from(val: &'a [u8]) -> Self {
        Self::new(val)
    }
}
