use std::io;
use crate::{value::Value, primitive::*, codec::Encode, constructor::Constructor};
pub trait Type<'a>
where
    Self: Encode + TryFrom<Value<'a>, Error = io::Error>,
    Self: TryFrom<Self::Source, Error = Self::Source> + Into<Self::Source>
{
    type Source: Type<'a>;
}

// a multiple type could be an element of an array 
pub trait Multiple<'a> {
    const CONSTRUCTOR: Constructor<'a>;
}
// impl<'a, T: Type> Type<'a> for List<'a, T> {
//     type Source = Self;
// }

// impl<'a, T: Type<'a>> Type<'a> for Array<'a, T> {
//     type Source = Self;
// }