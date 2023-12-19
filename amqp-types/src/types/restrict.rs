use crate::{primitive::{Array, Binary, Map, Symbol, Ts, Uuid}, Value};
pub trait Restrict: Sized {
    type Source;
    fn restrict(source: Self::Source) -> Result<Self, Self::Source>;
    fn source(self) -> Self::Source;
}

#[macro_export]
macro_rules! no_restrict {
    {$Type: ty} => {
        impl $crate::types::Restrict for $Type {
            type Source = Self;
            fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
                Ok(source)
            }
            fn source(self) -> Self::Source {
                self
            }
        }
    };
    {$($Type: ty)+} => {
        $($crate::no_restrict!{$Type})+
    };
}

no_restrict! {
    i8
    i16
    i32
    i64
    u8
    u16
    u32
    u64
    f32
    f64
    char
    Uuid
    Ts
    Symbol<'_>
    Binary<'_>
    &str
    Value<'_>
}

impl<'a, T: Restrict> Restrict for Array<'a, T> {
    type Source = Self;
    fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
        Ok(source)
    }
    fn source(self) -> Self::Source {
        self
    }
}

impl<'a, K: Restrict, V: Restrict> Restrict for Map<'a, K, V> {
    type Source = Self;
    fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
        Ok(source)
    }
    fn source(self) -> Self::Source {
        self
    }
}

impl<T: Restrict> Restrict for Option<T> {
    type Source = Self;
    fn restrict(source: Self::Source) -> Result<Self, Self::Source> {
        Ok(source)
    }
    fn source(self) -> Self::Source {
        self
    }
}