use std::borrow::Cow;

use serde::Serialize;

use super::{value::{Construct, Descriptor}, codes::FormatCode};

pub trait AmqpType: Serialize + Sized {
    const DESCRIPTOR: Option<Descriptor<'static>> = None;
    fn as_value(
        &self,
    ) -> Result<super::value::Value<'static>, crate::types::serde::ser::SerializeError> {
        let value = crate::types::serde::ser::as_value(self)?;
        if let Some(descriptor) = Self::DESCRIPTOR {
            Ok(super::value::Value::Described(Box::new(
                super::value::Described { descriptor, value },
            )))
        } else {
            Ok(value)
        }
    }
}

#[macro_export]
macro_rules! amqp_type {
    ($Type: ty , $($rest:tt)*) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::types::value::Descriptor<'static>> = None;
        }
        $crate::amqp_type!($($rest)*);
    };
    ($Type: ty) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::types::value::Descriptor<'static>> = None;
        }
    };
    ($Type: ty = $domain: literal: $code: literal, $($rest:tt)*) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::types::value::Descriptor<'static>> =
                Some($crate::types::value::Descriptor::Numeric($domain, $code));
        }
        $crate::amqp_type!($($rest)*);
    };
    ($Type: ty = $domain: literal: $code: literal) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::types::value::Descriptor<'static>> =
                Some($crate::types::value::Descriptor::Numeric($domain, $code));
        }
    };
    () => {

    }
}

amqp_type! {
    i8, i16, i32, i64, 
    u8, u16, u32, u64, 
    f32, f64, 
    bool, char, 
    &str, String,
}

pub struct Value<'a> {
    pub descriptor: Option<Descriptor<'a>>,
    pub format_code: FormatCode,
    pub value: Cow<'a, [u8]>,
}

// pub struct Describe<T: Described>(T);

// impl<T: Described + Construct> Construct for Describe<T> {
//     fn constructor() -> super::value::Constructor<'static> {
//         super::value::Constructor::Described {
//             descriptor: T::DESCRIPTOR,
//             constructor: Box::new(T::constructor()),
//         }
//     }
// }
