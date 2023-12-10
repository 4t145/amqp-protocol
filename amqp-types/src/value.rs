use bytes::Bytes;

use crate::{constructor::Constructor, data::Data, primitives::Primitive};
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    pub constructor: Constructor,
    pub data: Bytes,
}

impl Value {
    pub fn construct(&self) -> Option<Primitive> {
        let mut data = self.data.clone();
        self.constructor.construct(&mut data)
    }
}
