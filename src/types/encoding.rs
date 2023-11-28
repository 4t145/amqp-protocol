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

use std::{fmt, slice::Iter};

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
impl std::error::Error for DecodeErrorKind {}
impl fmt::Display for DecodeErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub type DecodeResult<T> = Result<T, DecodeErrorKind>;
pub enum Constructor<'a> {
    FormatCode(FormatCode),
    Described {
        descriptor: Box<Descriptor<'a>>,
        constructor: Box<Constructor<'a>>,
    },
}

impl<'a> Constructor<'a> {
    pub fn size(&self, data: &[u8]) -> DecodeResult<usize> {
        match self {
            Constructor::FormatCode(format_code) => format_code.size(data),
            Constructor::Described { constructor, .. } => constructor.size(data),
        }
    }
}

pub struct Descriptor<'a> {
    pub constructor: Constructor<'a>,
    pub untyped_bytes: &'a [u8],
}

pub struct Value<'a> {
    constructor: Constructor<'a>,
    untyped_bytes: &'a [u8],
}

pub trait Decode<'b>: Sized {
    fn try_decode(bytes: &'b [u8]) -> (DecodeResult<Self>, &'b [u8]);
}

// fn take_n<'a>(
//     bytes: &'a [u8],
//     n: usize,
//     expect: &'static str,
// ) -> (DecodeResult<&'a [u8]>, &'a [u8]) {
//     let (taked, rest) = bytes.split_at(n);
//     if taked.len() == n {
//         (Ok(taked), rest)
//     } else {
//         (Err(DecodeErrorKind::Expect(expect)), bytes)
//     }
// }

fn n_bytes<'a>(
    n: usize,
    expect: &'static str,
) -> impl Fn(&'a [u8]) -> (DecodeResult<&'a [u8]>, &'a [u8]) {
    move |bytes: &'a [u8]| {
        let (taked, rest) = bytes.split_at(n);
        if taked.len() == n {
            (Ok(taked), rest)
        } else {
            (Err(DecodeErrorKind::Expect(expect)), bytes)
        }
    }
}

fn u8<'a>(expect: &'static str) -> impl Fn(&'a [u8]) -> (DecodeResult<u8>, &'a [u8]) {
    move |bytes: &'a [u8]| {
        if let Some((u8, bytes)) = bytes.split_first() {
            (Ok(*u8), bytes)
        } else {
            (Err(DecodeErrorKind::Expect(expect)), bytes)
        }
    }
}

fn read_u8<'a>(bytes: &'a [u8], expect: &'static str) -> (DecodeResult<u8>, &'a [u8]) {
    if let Some((u8, bytes)) = bytes.split_first() {
        (Ok(*u8), bytes)
    } else {
        (Err(DecodeErrorKind::Expect(expect)), bytes)
    }
}

fn read_size1<'a>(bytes: &'a [u8]) -> (DecodeResult<u8>, &'a [u8]) {
    read_u8(bytes, "size1")
}

fn read_u32<'a>(bytes: &'a [u8], expect: &'static str) -> (DecodeResult<u32>, &'a [u8]) {
    let (n, bytes) = bytes.split_at(4);
    let Ok(n) = n.try_into() else {
        return (Err(DecodeErrorKind::Expect(expect)), bytes);
    };
    let n = u32::from_be_bytes(n);
    (Ok(n), bytes)
}

fn read_size4<'a>(bytes: &'a [u8]) -> (DecodeResult<u32>, &'a [u8]) {
    read_u32(bytes, "size4")
}

macro_rules! tri {
    ($T: ty, $bytes: expr) => {{
        let (result, bytes) = <$T>::try_decode($bytes);
        match result {
            Ok(result) => (result, bytes),
            Err(err) => return (Err(err), bytes),
        }
    }};
}

impl<'a, 'b: 'a> Decode<'b> for Constructor<'a> {
    fn try_decode(bytes: &'b [u8]) -> (DecodeResult<Self>, &'b [u8]) {
        let Some((&0x00, bytes)) = bytes.split_first() else {
            let (format_code, bytes) = tri!(FormatCode, bytes);
            return (Ok(Constructor::FormatCode(format_code)), bytes);
        };
        let (descriptor, bytes) = tri!(Descriptor, bytes);
        let (constructor, bytes) = tri!(Constructor, bytes);
        (
            Ok(Constructor::Described {
                descriptor: Box::new(descriptor),
                constructor: Box::new(constructor),
            }),
            bytes,
        )
    }
}

impl<'a, 'b: 'a> Decode<'b> for Descriptor<'a> {
    fn try_decode(bytes: &'b [u8]) -> (DecodeResult<Self>, &'b [u8]) {
        let (constructor, bytes) = tri!(Constructor, bytes);
        (
            Ok(Descriptor {
                constructor,
                untyped_bytes: bytes,
            }),
            bytes,
        )
    }
}
