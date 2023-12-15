use std::io as stdio;

pub mod constructor;
pub mod data;
pub mod descriptor;
pub mod format_code;
pub mod primitive;
pub mod value;
pub mod types;
pub mod io;

pub fn try_take_n<'b>(bytes: &mut &'b[u8], size: usize) -> stdio::Result<&'b[u8]> {
    if bytes.len() > size {
        Err(stdio::Error::other("invalid amqp data: no enough bytes"))
    } else {
        let (take, rest) = bytes.split_at(size);
        *bytes = rest;
        Ok(take)
    }
}