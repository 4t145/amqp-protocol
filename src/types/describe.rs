use super::value::Descriptor;

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