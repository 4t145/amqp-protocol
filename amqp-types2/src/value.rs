use std::io;

use crate::{constructor::Constructor, data::Data, primitive::Primitive};

#[derive(Debug, Clone,)]
pub struct Value<'frame> {
    constructor: Constructor<'frame>,
    data: Data<'frame>,
}

impl<'frame> Value<'frame>  {
    pub fn new(constructor: impl Into<Constructor<'frame>>, data: impl Into<Data<'frame>>) -> Self {
        Value {
            constructor: constructor.into(),
            data: data.into()
        }
    }
    pub fn construct(self) -> io::Result<Primitive<'frame>> {
        let Self { constructor, data } = self;
        constructor.construct(data)
    }
}