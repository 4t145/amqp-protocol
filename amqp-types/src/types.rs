use crate::{codec::Encode, constructor::Constructor, primitive::*, value::Value};
use std::io;

pub mod restrict;

pub trait Type<'a>: Encode<'a> + Restrict<'a> {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error>;
}
#[macro_export]
macro_rules! no_restrict {
    {} => {
        type Source = Self;
        fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
            Ok(source)
        }
        fn source(self) -> Self::Source {
            self
        }
    };
}
// a multiple type could be an element of an array
pub trait Multiple {}

impl<'a> Type<'a> for i8 {
    fn try_from_value(value: Value<'a>) -> Result<Self, io::Error> {
        todo!()
    }
}

// impl<'a, T: Type> Type<'a> for List<'a, T> {
//     type Source = Self;
// }

// impl<'a, T: Type<'a>> Type<'a> for Array<'a, T> {
//     type Source = Self;
// }

// macro_rules! derive_primitives {
//     ($($ty:ty)*) => {
//         $(
//             impl<'a> Type<'a> for $ty {
//                 no_restrict!{}
//             }
//             impl<'a> Multiple for $ty { }
//         )*
//     };
//     ({$lt:lifetime} $($ty:ident);* $(;)?) => {
//         $(
//             impl<$lt> Type<$lt> for $ty<$lt> {
//                 no_restrict!{}
//             }
//             impl<$lt> Multiple for $ty<$lt> { }

//         )*
//     };
// }

// derive_primitives! {
//     i8
//     i16
//     i32
//     i64
//     u8
//     u16
//     u32
//     u64
//     f32
//     f64
//     char
//     Uuid
//     Ts
// }

// derive_primitives! {
//     {'a}
//     Symbol

// }

// impl<'a> Type<'a> for &'a str {
//     no_restrict! {}
// }

// impl<'a> Multiple<'a> for &'a str {}

// impl<'a, T: Type<'a> + Multiple<'a>> Type<'a> for Array<'a, T> {
//     no_restrict! {}
// }
// impl<'a, T: Type<'a>> Multiple<'a> for Array<'a, T> {}
// impl<'a, T: Type<'a>> Type<'a> for Option<T> {
//     type Source = Option<T::Source>;
//     fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
//         Ok(source.map(T::restrict))
//     }
//     fn source(self) -> Self::Source {
//         self.map(T::source)
//     }
// }
