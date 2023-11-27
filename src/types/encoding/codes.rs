use super::{Decode, DecodeErrorKind, DecodeResult};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormatCode {
    Primitive(u8),
    Ext(u8, u8),
}

macro_rules! primitive {
    ($($name: ident = $code: expr,)*) => {
        $(
            pub const $name: FormatCode = FormatCode::Primitive($code);
        )*
    };
}

impl FormatCode {
    pub fn is_ext_type(&self) -> bool {
        match self {
            FormatCode::Ext(_, _) => true,
            _ => false,
        }
    }

    pub fn is_primitive_type(&self) -> bool {
        match self {
            FormatCode::Primitive(_) => true,
            _ => false,
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
        ULONG = 0x80,
        LONG = 0x81,
        FLOAT = 0x72,
        DOUBLE = 0x82,
        DECIMAL32 = 0x74,
        DECIMAL64 = 0x84,
        DECIMAL128 = 0x94,
        CHAR = 0x73,
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

pub enum Catagory {
    FixedWidth(FixedWidthSubcatagory),
    VariableWidth(VariableWidthSubcatagory),
    Compound(CompoundSubcatagory),
    Array(ArraySubcatagory),
}

pub enum FixedWidthSubcatagory {
    Zero,
    One,
    Four,
    Eight,
    Sixteen,
}

pub enum VariableWidthSubcatagory {
    One,
    Four,
}

pub enum CompoundSubcatagory {
    One,
    Four,
}

pub enum ArraySubcatagory {
    One,
    Four,
}

impl Decode<'_, '_> for FormatCode {
    fn try_decode(bytes: &mut &[u8]) -> DecodeResult<Self> {
        let Some((b0, new_bytes)) = bytes.split_first() else {
            return Err(DecodeErrorKind::Expect("format code"));
        };
        *bytes = new_bytes;
        if b0 & 0x10 != 0x0f {
            return Ok(FormatCode::Primitive(*b0));
        }
        let Some((b1, new_bytes)) = bytes.split_first() else {
            return Err(DecodeErrorKind::Expect("ext code"));
        };
        *bytes = new_bytes;
        Ok(FormatCode::Ext(*b0, *b1))
    }
}
