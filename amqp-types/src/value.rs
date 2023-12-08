use bytes::Bytes;

use crate::{constructor::Constructor, data::Data};

pub struct Value {
    pub constructor: Constructor,
    pub data: Data,
}

pub struct Symbol(Bytes);

impl Symbol {
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        Symbol(bytes.into())
    }
}
