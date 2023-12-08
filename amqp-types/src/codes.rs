use bytes::Bytes;
use serde::Deserialize;

use crate::{codec::BytesExt, constructor::Constructor};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatCode {
    Primitive(u8),
    Ext(u8, u8),
}

impl std::fmt::Debug for FormatCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(arg0) => f
                .debug_tuple("Primitive")
                .field(&format_args!("0x{:02X}", arg0))
                .finish(),
            Self::Ext(arg0, arg1) => f
                .debug_tuple("Ext")
                .field(&format_args!("0x{:02X}", arg0))
                .field(&format_args!("0x{:02X}", arg1))
                .finish(),
        }
    }
}

macro_rules! primitive {
    ($($name: ident = $code: expr,)*) => {
        $(
            pub const $name: FormatCode = FormatCode::Primitive($code);
        )*
    };
}

impl FormatCode {
    pub const fn into_u8(self) -> u8 {
        match self {
            FormatCode::Primitive(b) => b,
            FormatCode::Ext(b1, _) => b1,
        }
    }
    pub fn is_ext_type(&self) -> bool {
        matches!(self, FormatCode::Ext(_, _))
    }

    pub fn is_primitive_type(&self) -> bool {
        matches!(self, FormatCode::Primitive(_))
    }
    pub const fn category(&self) -> Category {
        let b1 = match self {
            FormatCode::Primitive(b) => b,
            FormatCode::Ext(b, _) => b,
        };
        match *b1 & 0xf0 {
            0x40 => Category::FixedWidth(FixedWidthSubcategory::Zero),
            0x50 => Category::FixedWidth(FixedWidthSubcategory::One),
            0x60 => Category::FixedWidth(FixedWidthSubcategory::Two),
            0x70 => Category::FixedWidth(FixedWidthSubcategory::Four),
            0x80 => Category::FixedWidth(FixedWidthSubcategory::Eight),
            0x90 => Category::FixedWidth(FixedWidthSubcategory::Sixteen),
            0xa0 => Category::VariableWidth(VariableWidthSubcategory::One),
            0xb0 => Category::VariableWidth(VariableWidthSubcategory::Four),
            0xc0 => Category::Compound(CompoundSubcategory::One),
            0xd0 => Category::Compound(CompoundSubcategory::Four),
            0xe0 => Category::Array(ArraySubcategory::One),
            0xf0 => Category::Array(ArraySubcategory::Four),
            code => Category::Unimplemented(code),
        }
    }
    primitive! {
        NULL = 0x40,
        BOOLEAN_TRUE = 0x41,
        BOOLEAN_FALSE = 0x42,
        UINT_0 = 0x43,
        ULONG_0 = 0x44,
        LIST0 = 0x45,
        BOOLEAN = 0x56,
        UBYTE = 0x50,
        BYTE = 0x51,
        SMALL_UINT = 0x52,
        SMALL_ULONG = 0x53,
        SMALL_INT = 0x54,
        SMALL_LONG = 0x55,
        USHORT = 0x60,
        SHORT = 0x61,
        UINT = 0x70,
        INT = 0x71,
        FLOAT = 0x72,
        CHAR = 0x73,
        DECIMAL32 = 0x74,
        ULONG = 0x80,
        LONG = 0x81,
        DOUBLE = 0x82,
        DECIMAL64 = 0x84,
        DECIMAL128 = 0x94,
        TIMESTAMP = 0x83,
        UUID = 0x98,
        BINARY8 = 0xa0,
        BINARY32 = 0xb0,
        STRING8_UTF8 = 0xa1,
        STRING32_UTF8 = 0xb1,
        SYMBOL8 = 0xa3,
        SYMBOL32 = 0xb3,
        LIST8 = 0xc0,
        LIST32 = 0xd0,
        MAP8 = 0xc1,
        MAP32 = 0xd1,
        ARRAY8 = 0xe0,
        ARRAY32 = 0xf0,
    }
}

pub enum Category {
    FixedWidth(FixedWidthSubcategory),
    VariableWidth(VariableWidthSubcategory),
    Compound(CompoundSubcategory),
    Array(ArraySubcategory),
    Unimplemented(u8),
}

pub enum FixedWidthSubcategory {
    Zero,
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}

pub enum VariableWidthSubcategory {
    One,
    Four,
}

pub enum CompoundSubcategory {
    One,
    Four,
}

pub enum ArraySubcategory {
    One,
    Four,
}

pub struct ArrayVisitor {
    constructor: Constructor,
    count: usize,
    rest: Bytes,
}

impl FormatCode {
    pub fn take_size(&self, bytes: &mut Bytes) -> Option<usize> {
        let fb = match self {
            FormatCode::Primitive(b) => b,
            FormatCode::Ext(b, _) => b,
        };
        match *fb {
            0x40..=0x4f => Some(0),
            0x50..=0x5f => Some(1),
            0x60..=0x6f => Some(2),
            0x70..=0x7f => Some(4),
            0x80..=0x8f => Some(8),
            0x90..=0x9f => Some(16),
            0xa0..=0xaf | 0xc0..=0xcf | 0xe0..=0xef => {
                let size = bytes.try_eat_n::<1>()?;
                Some(size[0] as usize)
            }
            0xb0..=0xbf | 0xd0..=0xdf | 0xf0..=0xff => {
                let size = bytes.try_eat_n::<4>().map(u32::from_be_bytes)?;
                Some(size as usize)
            }
            _ => {
                // invalid code
                None
            }
        }
    }
    pub fn visit<'data, T: Deserialize<'data>>(&self, data: &'data [u8]) -> T {
        todo!()
    }

    pub fn visit_u8(&self, data: Bytes) -> Option<u8> {
        (data.len() >= std::mem::size_of::<u8>() && matches!(self, &FormatCode::UBYTE))
            .then_some(data[0])
    }

    pub fn visit_u16(&self, data: Bytes) -> Option<u16> {
        (data.len() >= std::mem::size_of::<u16>() && matches!(self, &FormatCode::USHORT))
            .then_some(u16::from_be_bytes([data[0], data[1]]))
    }

    pub fn visit_u32(&self, data: Bytes) -> Option<u32> {
        (data.len() >= std::mem::size_of::<u32>() && matches!(self, &FormatCode::UINT))
            .then_some(u32::from_be_bytes([data[0], data[1], data[2], data[3]]))
    }

    pub fn visit_u64(&self, data: Bytes) -> Option<u64> {
        (data.len() >= std::mem::size_of::<u64>() && matches!(self, &FormatCode::ULONG)).then_some(
            u64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]),
        )
    }

    pub fn visit_i8(&self, data: Bytes) -> Option<i8> {
        (data.len() >= std::mem::size_of::<i8>() && matches!(self, &FormatCode::BYTE))
            .then_some(data[0] as i8)
    }

    pub fn visit_i16(&self, data: Bytes) -> Option<i16> {
        (data.len() >= std::mem::size_of::<i16>() && matches!(self, &FormatCode::SHORT))
            .then_some(i16::from_be_bytes([data[0], data[1]]))
    }

    pub fn visit_i32(&self, data: Bytes) -> Option<i32> {
        (data.len() >= std::mem::size_of::<i32>() && matches!(self, &FormatCode::INT))
            .then_some(i32::from_be_bytes([data[0], data[1], data[2], data[3]]))
    }

    pub fn visit_i64(&self, data: Bytes) -> Option<i64> {
        (data.len() >= std::mem::size_of::<i64>() && matches!(self, &FormatCode::LONG)).then_some(
            i64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]),
        )
    }

    pub fn visit_f32(&self, data: Bytes) -> Option<f32> {
        (data.len() >= std::mem::size_of::<f32>() && matches!(self, &FormatCode::FLOAT))
            .then_some(f32::from_be_bytes([data[0], data[1], data[2], data[3]]))
    }

    pub fn visit_f64(&self, data: Bytes) -> Option<f64> {
        (data.len() >= std::mem::size_of::<f64>() && matches!(self, &FormatCode::DOUBLE)).then_some(
            f64::from_be_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]),
        )
    }

    pub fn visit_char(&self, data: Bytes) -> Option<char> {
        (data.len() >= std::mem::size_of::<char>() && matches!(self, &FormatCode::CHAR))
            .then_some(char::from_u32(u32::from_be_bytes([
                data[0], data[1], data[2], data[3],
            ])))
            .flatten()
    }
}

pub trait ExtCode {
    fn size_hint(data: &Bytes) -> Option<usize>;
}

impl ExtCode for () {
    fn size_hint(data: &Bytes) -> Option<usize> {
        panic!("not support ext code")
    }
}
