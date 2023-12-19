use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Default)]
pub struct Data<'a> {
    inner: &'a [u8],
}

impl<'a> Deref for Data<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Data<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
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

impl<'a> fmt::Debug for Data<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for byte_group in self.inner.chunks(16) {
            for byte in byte_group {
                write!(f, "{:02x} ", byte)?;
            }
            writeln!(f)?;
            for byte in byte_group {
                if byte.is_ascii_graphic() {
                    write!(f, " {} ", *byte as char)?;
                } else {
                    write!(f, ".  ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
