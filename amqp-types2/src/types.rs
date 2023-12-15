use std::io;
use crate::{value::Value, primitive::*};
pub trait Type<'a>
where
    Self: Into<Value<'a>> + TryFrom<Value<'a>, Error = io::Error>,
    Self: TryFrom<Self::Source, Error = Self::Source> + Into<Self::Source>
{
    type Source: Type<'a>;
}

// impl<'a, T: Type> Type<'a> for List<'a, T> {
//     type Source = Self;
// }