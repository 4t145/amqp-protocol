use bytes::Bytes;

use crate::{primitives::Symbol, value::Value};
#[derive(Debug, Clone)]
pub enum Descriptor {
    Symbol(Symbol),
    Numeric(u64),
    Reserved(Box<Value>),
}

impl Descriptor {
    pub const fn symbol(bytes: &'static [u8]) -> Self {
        Self::Symbol(Symbol(Bytes::from_static(bytes)))
    }
    pub const fn numeric(id: u64) -> Self {
        Self::Numeric(id)
    }
}
