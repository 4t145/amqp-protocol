use std::ops::{Deref, DerefMut};

use bytes::Bytes;

use crate::{
    codec::{BytesExt, Decode},
    codes::FormatCode,
    constructor::Constructor,
    descriptor::Descriptor,
    value::{Value},
};

#[derive(Debug, Clone)]
pub struct Data(pub Bytes);

pub trait DataView: Sized {
    fn view(constructor: &Constructor, bytes: &mut Bytes) -> Option<Self>;
    // let size = constructor.format_code.take_size(bytes)?;
    // let data = bytes.try_eat(size)?;
    // Some(Self::decode(&mut data)?)
}

impl Deref for Data {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
