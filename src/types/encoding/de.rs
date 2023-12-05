use std::{fmt, io, mem::size_of, slice::Iter, string};

use crate::types::codes::FormatCode;

use crate::types::value::{Constructor, Described, Descriptor, Primitive, Symbol, Value};
pub mod async_reader;
pub mod reader;
pub mod slice;

pub(super) struct ArrayIter<'c> {
    pub(crate) count: usize,
    pub(crate) constructor: Constructor<'c>,
}

impl<'c> ArrayIter<'c> {
    pub fn new(count: usize, constructor: Constructor<'c>) -> Self {
        ArrayIter { count, constructor }
    }
}

pub(crate) struct CompoundIter {
    pub(crate) count: usize,
}

impl CompoundIter {
    pub fn new(count: usize) -> Self {
        CompoundIter { count }
    }
}
