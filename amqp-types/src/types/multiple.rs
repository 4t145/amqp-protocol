use crate::primitive::*;

/// Most AMQP types should be multiple except those types could be null.
pub trait Multiple {}

macro_rules! multiple {
    {$($Type: ty)*} => {
        $(impl Multiple for $Type{})*
    };
}

multiple! {
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
}

impl<'a, T> Multiple for Array<'a, T> {}
impl<'a, K, V> Multiple for Map<'a, K, V> {}
