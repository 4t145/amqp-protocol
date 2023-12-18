use std::io;

use bytes::Bytes;

use crate::{
    codec::DecodeExt,
    error::{UNEXPECTED_TYPE, UNKNOWN_AMQP_TYPE},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatCode {
    Primitive(u8),
    Ext(u8, u8),
}

impl Default for FormatCode {
    fn default() -> Self {
        Self::NULL
    }
}

impl std::fmt::Debug for FormatCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NULL => f.write_str("NULL"),
            Self::BOOLEAN_TRUE => f.write_str("BOOLEAN_TRUE"),
            Self::BOOLEAN_FALSE => f.write_str("BOOLEAN_FALSE"),
            Self::UINT_0 => f.write_str("UINT_0"),
            Self::ULONG_0 => f.write_str("ULONG_0"),
            Self::LIST0 => f.write_str("LIST0"),
            Self::BOOLEAN => f.write_str("BOOLEAN"),
            Self::UBYTE => f.write_str("UBYTE"),
            Self::BYTE => f.write_str("BYTE"),
            Self::SMALL_UINT => f.write_str("SMALL_UINT"),
            Self::SMALL_ULONG => f.write_str("SMALL_ULONG"),
            Self::SMALL_INT => f.write_str("SMALL_INT"),
            Self::SMALL_LONG => f.write_str("SMALL_LONG"),
            Self::USHORT => f.write_str("USHORT"),
            Self::SHORT => f.write_str("SHORT"),
            Self::UINT => f.write_str("UINT"),
            Self::INT => f.write_str("INT"),
            Self::FLOAT => f.write_str("FLOAT"),
            Self::CHAR => f.write_str("CHAR"),
            Self::DECIMAL32 => f.write_str("DECIMAL32"),
            Self::ULONG => f.write_str("ULONG"),
            Self::LONG => f.write_str("LONG"),
            Self::DOUBLE => f.write_str("DOUBLE"),
            Self::DECIMAL64 => f.write_str("DECIMAL64"),
            Self::DECIMAL128 => f.write_str("DECIMAL128"),
            Self::TIMESTAMP => f.write_str("TIMESTAMP"),
            Self::UUID => f.write_str("UUID"),
            Self::BINARY8 => f.write_str("BINARY8"),
            Self::BINARY32 => f.write_str("BINARY32"),
            Self::STRING8_UTF8 => f.write_str("STRING8_UTF8"),
            Self::STRING32_UTF8 => f.write_str("STRING32_UTF8"),
            Self::SYMBOL8 => f.write_str("SYMBOL8"),
            Self::SYMBOL32 => f.write_str("SYMBOL32"),
            Self::LIST8 => f.write_str("LIST8"),
            Self::LIST32 => f.write_str("LIST32"),
            Self::MAP8 => f.write_str("MAP8"),
            Self::MAP32 => f.write_str("MAP32"),
            Self::ARRAY8 => f.write_str("ARRAY8"),
            Self::ARRAY32 => f.write_str("ARRAY32"),
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

impl FormatCode {
    pub fn peek_size(&self, bytes: &[u8]) -> io::Result<usize> {
        let fb = match self {
            FormatCode::Primitive(b) => b,
            FormatCode::Ext(b, _) => b,
        };
        let size = match *fb {
            0x40..=0x4f => 0,
            0x50..=0x5f => 1,
            0x60..=0x6f => 2,
            0x70..=0x7f => 4,
            0x80..=0x8f => 8,
            0x90..=0x9f => 16,
            0xa0..=0xaf | 0xc0..=0xcf | 0xe0..=0xef => {
                let size = bytes.peek_n::<1>()?;
                size[0] as usize + 1
            }
            0xb0..=0xbf | 0xd0..=0xdf | 0xf0..=0xff => {
                let size = bytes.peek_n::<4>().map(u32::from_be_bytes)?;
                size as usize + 4
            }
            unknown => {
                // invalid code
                return Err(io::Error::other(format!(
                    "unknown format code: 0x{:02X}",
                    unknown
                )));
            }
        };
        Ok(size)
    }
}
