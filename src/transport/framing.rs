use bytes::{BufMut, Bytes, BytesMut};
use std::io::{self, Read, Write};
pub struct FrameHeader {
    pub size: u32,
    pub doff: u8,
    pub frame_type: u8,
    pub ext: u16,
}

impl FrameHeader {
    pub fn exthdr_size(&self) -> Option<usize> {
        (self.doff as usize).checked_mul(4)?.checked_sub(8)
    }
    pub fn body_size(&self) -> Option<usize> {
        (self.size as usize).checked_sub((self.doff as usize).checked_mul(4)?)
    }
}

pub struct Frame<'f, Ext, Body> {
    header: &'f FrameHeader,
    extended_header: &'f Ext,
    body: Body,
}
