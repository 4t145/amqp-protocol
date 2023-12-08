use std::ops::{Deref, DerefMut};

use bytes::Bytes;

use crate::{constructor::Constructor, value::Value};

pub struct Data(pub Bytes);

pub trait DataView: Sized {
    fn accept(constructor: &Constructor) -> bool {
        let _ = constructor;
        true
    }
    fn view(data: &mut Bytes) -> Option<Self>;
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

impl DataView for Data {
    fn view(data: &mut Bytes) -> Option<Self> {
        let len = data.len();
        Some(Data(data.split_to(len)))
    }
}




pub struct ArrayView<T> {
    data: Data,
    constructor: Constructor,
    rest: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DataView> Iterator for ArrayView<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest == 0 {
            return None;
        }
        self.rest -= 1;
        self.constructor.construct(&mut self.data)
    }
}
