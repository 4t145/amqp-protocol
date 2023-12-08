use crate::value::{Symbol, Value};

pub enum Descriptor {
    Symbol(Symbol),
    Numeric(u32, u32),
    Reserved(Box<Value>),
}

pub trait AmqpType: Sized {
    const DESCRIPTOR: Option<Descriptor> = None;
    // fn as_value(
    //     &self,
    // ) -> Result<super::value::Value<'static>, crate::types::serde::ser::SerializeError> {
    //     let value = crate::types::serde::ser::as_value(self)?;
    //     if let Some(descriptor) = Self::DESCRIPTOR {
    //         Ok(super::value::Value::Described(Box::new(
    //             super::value::Described { descriptor, value },
    //         )))
    //     } else {
    //         Ok(value)
    //     }
    // }
}

#[macro_export]
macro_rules! amqp_type {
    ($Type: ty , $($rest:tt)*) => {
        impl $crate::descriptor::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::descriptor::Descriptor> = None;
        }
        $crate::amqp_type!($($rest)*);
    };
    ($Type: ty) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::descriptor::Descriptor> = None;
        }
    };
    ($Type: ty = $domain: literal: $code: literal, $($rest:tt)*) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::descriptor::Descriptor> =
                Some($crate::descriptor::Descriptor::Numeric($domain, $code));
        }
        $crate::amqp_type!($($rest)*);
    };
    ($Type: ty = $domain: literal: $code: literal) => {
        impl $crate::types::describe::AmqpType for $Type {
            const DESCRIPTOR: Option<$crate::descriptor::Descriptor> =
                Some($crate::descriptor::Descriptor::Numeric($domain, $code));
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
