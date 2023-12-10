use std::io;

use bytes::{BufMut, BytesMut};
use tokio::io::{AsyncWrite};

use crate::{
    codes::FormatCode, constructor::Constructor, descriptor::Descriptor, primitives::Symbol,
    value::Value,
};

pub trait Encode {
    fn encode(&self, bytes: &mut BytesMut);
}

impl Encode for FormatCode {
    fn encode(&self, bytes: &mut BytesMut) {
        match self {
            FormatCode::Primitive(p) => bytes.put_u8(*p),
            FormatCode::Ext(c, e) => {
                bytes.put_u8(*c);
                bytes.put_u8(*e);
            }
        }
    }
}

impl Encode for Symbol {
    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put(self.0.clone());
    }
}

impl Encode for Descriptor {
    fn encode(&self, bytes: &mut BytesMut) {
        match self {
            Descriptor::Symbol(s) => s.encode(bytes),
            Descriptor::Numeric(n) => bytes.put_u64(*n),
            Descriptor::Reserved(_) => todo!(),
        }
    }
}

impl Encode for Constructor {
    fn encode(&self, bytes: &mut BytesMut) {
        if let Some(descriptor) = &self.descriptor {
            bytes.put_u8(0x00);
            descriptor.encode(bytes);
        }
        self.format_code.encode(bytes);
    }
}

impl Encode for Value {
    fn encode(&self, bytes: &mut BytesMut) {
        self.constructor.encode(bytes);
        bytes.put(self.data.clone());
    }
}
