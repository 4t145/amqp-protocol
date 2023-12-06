use super::value::{Descriptor, Construct};

pub trait Described {
    const DESCRIPTOR: Descriptor<'static>;
}

#[macro_export]
macro_rules! derive_descriptor {
    ($Type: ty = $domain: literal: $code: literal) => {
        impl $crate::types::describe::Described for $Type {
            const DESCRIPTOR: $crate::types::value::Descriptor<'static> = $crate::types::value::Descriptor::Numeric($domain, $code);
        }
    };
}

pub struct Describe<T: Described>(T);

impl<T: Described + Construct> Construct for Describe<T> {
    fn constructor() -> super::value::Constructor<'static> {
        super::value::Constructor::Described {
            descriptor: T::DESCRIPTOR,
            constructor: Box::new(T::constructor()),
        }
    }
}