pub mod de;

use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
};

use crate::primitive::Array;

pub trait Encode {
    fn encode(self, buffer: &mut [u8]) -> io::Result<()>;
}

impl Encode for &[u8] {
    fn encode(self, mut buffer: &mut [u8]) -> io::Result<()> {
        buffer.write_all(self)
    }
}

// impl<'x, T: WriteAble> for Array<T>