
use bytes::{BufMut, BytesMut};

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


impl Encode for bool {
    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u8(if *self { 0x01 } else { 0x00 });
    }
}

impl Encode for char {
    fn encode(&self, bytes: &mut BytesMut) {
        bytes.put_u32(*self as u32);
    }
}

macro_rules! derive_primitives {
    ($($ty: ty, $f: ident)*) => {
        $(impl Encode for $ty {
            fn encode(&self, bytes: &mut BytesMut) {
                bytes.$f(*self);
            }
        })*
    };
}

derive_primitives! {
    u8, put_u8
    u16, put_u16
    u32, put_u32
    u64, put_u64
    i8, put_i8
    i16, put_i16
    i32, put_i32
    i64, put_i64
    f32, put_f32
    f64, put_f64
}
