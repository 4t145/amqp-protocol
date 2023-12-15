use bytes::Bytes;

use crate::{primitives::Symbol, value::Value};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Descriptor<'frame> {
    Symbol(Symbol),
    Numeric(u64),
    Reserved(Value<'frame>),
}

impl<'frame> Descriptor<'frame> {
    pub const fn symbol(bytes: &'static [u8]) -> Self {
        Self::Symbol(Symbol(Bytes::from_static(bytes)))
    }
    pub const fn numeric(id: u64) -> Self {
        Self::Numeric(id)
    }
}
