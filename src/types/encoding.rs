// constructor = format-code
// / %x00 descriptor constructor
// format-code = fixed / variable / compound / array
// fixed = empty / fixed-one / fixed-two / fixed-four
// / fixed-eight / fixed-sixteen
// variable = variable-one / variable-four
// compound = compound-one / compound-four
// array = array-one / array-four
// descriptor = value
// value = constructor untyped-bytes
// untyped-bytes = *OCTET ; this is not actually *OCTET, the
// ; valid byte sequences are restricted
// ; by the constructor
// ; fixed width format codes
// empty = %x40-4E / %x4F %x00-FF
// fixed-one = %x50-5E / %x5F %x00-FF
// fixed-two = %x60-6E / %x6F %x00-FF
// fixed-four = %x70-7E / %x7F %x00-FF
// fixed-eight = %x80-8E / %x8F %x00-FF
// fixed-sixteen = %x90-9E / %x9F %x00-FF
// ; variable width format codes
// variable-one = %xA0-AE / %xAF %x00-FF
// variable-four = %xB0-BE / %xBF %x00-FF
// ; compound format codes
// compound-one = %xC0-CE / %xCF %x00-FF
// compound-four = %xD0-DE / %xDF %x00-FF
// ; array format codes
// array-one = %xE0-EE / %xEF %x00-FF
// array-four = %xF0-FE / %xFF %x00-FF

use std::{slice::Iter, fmt};

use self::codes::FormatCode;
mod codes;
mod de;

#[derive(Debug)]
pub enum DecodeErrorKind {
    Expect(&'static str),
    Invalid(&'static str, u8),
}

impl serde::de::Error for DecodeErrorKind {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        todo!()
    }
}
impl std::error::Error for DecodeErrorKind {

}
impl fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub type DecodeResult<T> = Result<T, DecodeErrorKind>;
pub enum Constructor<'a> {
    FormatCode(FormatCode),
    Described {
        descriptor: &'a Descriptor<'a>,
        constructor: &'a Constructor<'a>
    }
}

impl<'a>  Constructor<'a> {

}

pub struct Descriptor<'a> {
    pub constructor: Constructor<'a>,
    pub untyped_bytes: &'a [u8],
}

pub struct Value<'a> {
    constructor: Constructor<'a>,
    untyped_bytes: &'a [u8],
}


pub trait Decode<'a, 'b>: Sized {
    fn try_decode(bytes: &'a mut &'b [u8]) -> DecodeResult<Self>;
}


fn read_u8(bytes: &mut &[u8], expect: &'static str) -> DecodeResult<u8> {
    if let Some((u8, new_bytes)) = bytes.split_first() {
        *bytes = new_bytes;
        Ok(*u8)
    } else {
        Err(DecodeErrorKind::Expect(expect))
    }
}

fn read_size1(bytes: &mut &[u8]) -> DecodeResult<u8> {
    read_u8(bytes, "size1")
}

fn read_u32(bytes: &mut &[u8], expect: &'static str) -> DecodeResult<u32> {
    let (n, new_bytes) = bytes.split_at(4);                     
    let Ok(n) = n.try_into() else {
        return Err(DecodeErrorKind::Expect(expect))
    };
    *bytes = new_bytes;
    let n = u32::from_be_bytes(n);
    Ok(n)
}

fn read_size4(bytes: &mut &[u8]) -> DecodeResult<u32> {
    read_u32(bytes, "size4")
}

impl<'a, 'b: 'a> Decode<'a, 'b> for Constructor<'a> {
    fn try_decode(bytes: &'a mut &'b [u8]) -> DecodeResult<Self> {
        let Some((&0x00, new_bytes)) = bytes.split_first() else {
            let format_code= FormatCode::try_decode(bytes)?;
            return Ok(Constructor::FormatCode(format_code))
        };
        *bytes = new_bytes;
        let descriptor = Descriptor::try_decode(bytes)?;
        let mut bytes = bytes.clone();
        let constructor = Constructor::try_decode(&mut bytes)?;
        Ok(Constructor::Described { descriptor: &descriptor, constructor: &constructor }, )
    }
}

impl<'a, 'b> Decode<'a, 'b> for Descriptor<'a> {
    fn try_decode(bytes: &'a mut &'b [u8]) -> DecodeResult<Self> {
        let constructor = Constructor::try_decode(bytes)?;
        Ok(Descriptor {
            constructor,
            untyped_bytes: bytes.clone(),
        })
    }
}
