// not a key part



const MASK_SIGN: u32 = 0x8000_0000;
const MASK_COMBINATION_FIELD: u32 = 0x7fc0_0000;
const MASK_EXPONENT_FIELD: u32 = 0x0000_ffff;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[allow(non_camel_case_types)]
pub struct d32(u32);


impl d32 {
    pub const ONE: Self = Self(0x3f80_0000);
    pub const ZERO: Self = Self(0x0000_0000);
    pub const NEGATIVE_ZERO: Self = Self(0x8000_0000);
    pub const INFINITY: Self = Self(0x7f80_0000);
    pub const NEGATIVE_INFINITY: Self = Self(0xff80_0000);
    pub const NAN: Self = Self(0x7fc0_0000);
    pub const NEGATIVE_NAN: Self = Self(0xffc0_0000);
    pub const MIN: Self = Self(0x0000_0001);
    pub const MAX: Self = Self(0x7f7f_ffff);
    pub const NEGATIVE_MIN: Self = Self(0x8000_0001);
    pub const NEGATIVE_MAX: Self = Self(0xff7f_ffff);
    pub const MIN_POSITIVE_NORMAL: Self = Self(0x0080_0000);
    pub fn new(int: i32, dec: u32) -> Self {
        let sign = if int < 0 { MASK_SIGN } else { 0 };
        let int = int.unsigned_abs();
        let exp = 0;
        let mantissa = int << 8 | dec;
        Self(sign | exp << 23 | mantissa)
    }
}