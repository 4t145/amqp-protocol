use std::{
    fmt::Debug,
    io::{self, Cursor, Write},
};

pub trait WriteAble: Debug {
    fn write(&self, buffer: &mut [u8]) -> io::Result<()>;
}

impl WriteAble for &[u8] {
    fn write(&self, mut buffer: &mut [u8]) -> io::Result<()> {
        buffer.write_all(self)
    }
}
