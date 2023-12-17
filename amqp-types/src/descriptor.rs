use bytes::Bytes;

use crate::{primitive::Symbol, value::Value};
#[derive(Debug, Clone)]
pub enum Descriptor<'a> {
    Symbol(Symbol<'a>),
    Numeric(u64),
    Reserved(/* Box<Value<'frame>> */),
}

impl<'a> Descriptor<'a> {
    pub const fn symbol(bytes: &'static [u8]) -> Self {
        Self::Symbol(Symbol::new(bytes))
    }
    pub const fn numeric(id: u64) -> Self {
        Self::Numeric(id)
    }
}
