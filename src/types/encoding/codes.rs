use super::{read_size1, read_size4, Decode, DecodeErrorKind, DecodeResult};
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
        matches!(self, FormatCode::Ext(_, _))
    }

    pub fn is_primitive_type(&self) -> bool {
        matches!(self, FormatCode::Primitive(_))
    }
    pub const fn catagory(&self) -> Catagory {
        let b1 = match self {
            FormatCode::Primitive(b) => b,
            FormatCode::Ext(b, _) => b,
        };
        match *b1 & 0xf0 {
            0x40 => Catagory::FixedWidth(FixedWidthSubcatagory::Zero),
            0x50 => Catagory::FixedWidth(FixedWidthSubcatagory::One),
            0x60 => Catagory::FixedWidth(FixedWidthSubcatagory::Two),
            0x70 => Catagory::FixedWidth(FixedWidthSubcatagory::Four),
            0x80 => Catagory::FixedWidth(FixedWidthSubcatagory::Eight),
            0x90 => Catagory::FixedWidth(FixedWidthSubcatagory::Sixteen),
            0xa0 => Catagory::VariableWidth(VariableWidthSubcatagory::One),
            0xb0 => Catagory::VariableWidth(VariableWidthSubcatagory::Four),
            0xc0 => Catagory::Compound(CompoundSubcatagory::One),
            0xd0 => Catagory::Compound(CompoundSubcatagory::Four),
            0xe0 => Catagory::Array(ArraySubcatagory::One),
            0xf0 => Catagory::Array(ArraySubcatagory::Four),
            code => Catagory::Unimplemented(code),
        }
    }
    pub fn size(&self, data: &[u8]) -> DecodeResult<usize> {
        match self.catagory() {
            Catagory::FixedWidth(FixedWidthSubcatagory::Zero) => Ok(0),
            Catagory::FixedWidth(FixedWidthSubcatagory::One) => Ok(1),
            Catagory::FixedWidth(FixedWidthSubcatagory::Two) => Ok(2),
            Catagory::FixedWidth(FixedWidthSubcatagory::Four) => Ok(4),
            Catagory::FixedWidth(FixedWidthSubcatagory::Eight) => Ok(8),
            Catagory::FixedWidth(FixedWidthSubcatagory::Sixteen) => Ok(16),
            Catagory::VariableWidth(VariableWidthSubcatagory::One)
            | Catagory::Compound(CompoundSubcatagory::One)
            | Catagory::Array(ArraySubcatagory::One) => {
                read_size1(data).0.map(|s| (s as usize) + 1)
            }
            Catagory::VariableWidth(VariableWidthSubcatagory::Four)
            | Catagory::Compound(CompoundSubcatagory::Four)
            | Catagory::Array(ArraySubcatagory::Four) => {
                read_size4(data).0.map(|s| (s as usize) + 4)
            }
            Catagory::Unimplemented(code) => {
                Err(DecodeErrorKind::Invalid("unimplemented subcatagory", code))
            }
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
    Unimplemented(u8),
}

pub enum FixedWidthSubcatagory {
    Zero,
    One,
    Two,
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

impl Decode<'_> for FormatCode {
    fn try_decode(bytes: &[u8]) -> (DecodeResult<Self>, &[u8]) {
        let Some((b0, bytes)) = bytes.split_first() else {
            return (Err(DecodeErrorKind::Expect("format code")), bytes);
        };
        if b0 % 0x10 != 0x0f {
            return (Ok(FormatCode::Primitive(*b0)), bytes);
        }
        let Some((b1, bytes)) = bytes.split_first() else {
            return (Err(DecodeErrorKind::Expect("ext code")), bytes);
        };
        (Ok(FormatCode::Ext(*b0, *b1)), bytes)
    }
}
