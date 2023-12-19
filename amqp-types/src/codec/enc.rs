use std::{fmt::Debug, io, mem::size_of};

use crate::{
    constructor::Constructor,
    descriptor::Descriptor,
    format_code::FormatCode,
    primitive::{Array, Binary, Map, Symbol, Ts, Uuid},
    types::{Multiple, Type},
    value::Value,
};
#[derive(Debug)]
pub struct Writer<'a> {
    marker: std::marker::PhantomData<&'a mut [u8]>,
    from: *const u8,
    to: *const u8,
    w_ptr: *mut u8,
}

macro_rules! write_be {
    ($($f: ident: $T: ty)*) => {
        $(
            pub fn $f(&mut self, value: $T) -> io::Result<()> {
                self.remaining(size_of::<$T>())?;
                unsafe {
                    self.w_ptr
                        .copy_from_nonoverlapping(&value.to_be_bytes() as *const u8, size_of::<$T>());
                    self.w_ptr = self.w_ptr.add(size_of::<$T>());
                }
                Ok(())
            }
        )*
    };
}

impl<'a> Writer<'a> {
    pub fn new(writer: &'a mut [u8]) -> Self {
        let from = writer.as_ptr();
        let to = unsafe { from.add(writer.len()) };
        let w_ptr = writer.as_mut_ptr();
        Self {
            marker: std::marker::PhantomData,
            from,
            to,
            w_ptr,
        }
    }
    pub fn finish(self) -> &'a mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.from as *mut u8,
                self.to.offset_from(self.from) as usize,
            )
        }
    }
    pub fn remaining(&self, count: usize) -> io::Result<()> {
        if unsafe { self.w_ptr.add(count) as *const _ } <= self.to {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::WriteZero, "no enough space"))
        }
    }
    pub fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.remaining(1)?;
        unsafe {
            self.w_ptr.write(value);
            self.w_ptr = self.w_ptr.add(1);
        }
        Ok(())
    }
    write_be! {
        write_i8: i8
        write_i16: i16
        write_i32: i32
        write_i64: i64
        write_u16: u16
        write_u32: u32
        write_u64: u64
        write_f32: f32
        write_f64: f64
    }
    #[inline]
    pub fn write_char(&mut self, value: char) -> io::Result<()> {
        self.write_u32(value as u32)
    }
    #[inline]
    pub fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(if value { 1 } else { 0 })
    }
    pub fn write_slice(&mut self, value: &[u8]) -> io::Result<()> {
        let count = value.len();
        self.remaining(count)?;
        unsafe {
            self.w_ptr.copy_from_nonoverlapping(value.as_ptr(), count);
            self.w_ptr = self.w_ptr.add(count);
        }
        Ok(())
    }
    pub fn write_variable_8(&mut self, value: &[u8]) -> io::Result<()> {
        debug_assert!(u8::try_from(value.len()).is_ok());
        self.write_u8(value.len() as _)?;
        self.write_slice(value)?;
        Ok(())
    }
    pub fn write_variable_32(&mut self, value: &[u8]) -> io::Result<()> {
        debug_assert!(u32::try_from(value.len()).is_ok());
        self.write_u32(value.len() as _)?;
        self.write_slice(value)?;
        Ok(())
    }
    pub fn write_items_8(
        &mut self,
        fwrite: impl FnOnce(&mut Self) -> io::Result<usize>,
    ) -> io::Result<()> {
        let size_ptr = self.w_ptr;
        self.write_u8(0)?;
        let count_ptr = self.w_ptr;
        self.write_u8(0)?;
        let count = fwrite(self)?;
        unsafe {
            let size = self.w_ptr.byte_offset_from(count_ptr as _) as usize;
            debug_assert!(u8::try_from(size).is_ok());
            debug_assert!(u8::try_from(count).is_ok());
            size_ptr.write(size as u8);
            count_ptr.write(count as u8);
        }
        Ok(())
    }
    pub fn write_items_32(
        &mut self,
        fwrite: impl FnOnce(&mut Self) -> io::Result<usize>,
    ) -> io::Result<()> {
        let size_ptr = self.w_ptr;
        self.write_u32(0)?;
        let count_ptr = self.w_ptr;
        self.write_u32(0)?;
        let count = fwrite(self)?;
        unsafe {
            let size = dbg!(self.w_ptr.byte_offset_from(count_ptr as _) as usize);
            debug_assert!(u32::try_from(size).is_ok());
            debug_assert!(u32::try_from(count).is_ok());
            (size_ptr as *mut u32).write((size as u32).to_be());
            (count_ptr as *mut u32).write((count as u32).to_be());
        }
        Ok(())
    }
    pub fn write_constructor(&mut self, constructor: Constructor) -> io::Result<()> {
        if let Some(descriptor) = constructor.descriptor {
            self.write_u8(0x00)?;
            self.write_descriptor(descriptor)?
        }
        self.write_format_code(constructor.format_code)
    }
    pub fn write_format_code(&mut self, format_code: FormatCode) -> io::Result<()> {
        match format_code {
            FormatCode::Primitive(p) => self.write_u8(p),
            FormatCode::Ext(c, e) => {
                self.write_u8(c)?;
                self.write_u8(e)
            }
        }
    }
    pub fn write_descriptor(&mut self, descriptor: Descriptor) -> io::Result<()> {
        match descriptor {
            Descriptor::Symbol(s) => self.write_amqp_value(s)?,
            Descriptor::Numeric(n) => self.write_amqp_value(n)?,
            Descriptor::Reserved() => {
                unimplemented!();
            }
        };
        Ok(())
    }
    #[inline]
    pub fn write_amqp_value(&mut self, value: impl Encode) -> io::Result<()> {
        value.encode_default(self)
    }
}

pub trait Encode: Sized {
    const DESCRIPTOR: Option<Descriptor<'static>> = None;
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode;
    /// we should panic here if the format code is invalid since the format code is determained by ourselves.
    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()>;
    fn encode(self, constructor: Constructor, writer: &mut Writer) -> io::Result<()> {
        let format_code = constructor.format_code;
        writer.write_constructor(constructor)?;
        self.encode_data(format_code, writer)
    }
    fn encode_default(self, writer: &mut Writer) -> io::Result<()> {
        writer.write_constructor(Constructor {
            descriptor: Self::DESCRIPTOR,
            format_code: Self::ENCODE_DEFAULT_FORMAT_CODE,
        })?;
        self.encode_data(Self::ENCODE_DEFAULT_FORMAT_CODE, writer)
    }
}

// fn write_descriptor<'b>(writer: &'b mut [u8], descriptor: Descriptor) -> io::Result<&'b mut [u8]> {
//     match descriptor {
//         Descriptor::Symbol(s) => s.encode_default(writer)?,
//         Descriptor::Numeric(n) => n.encode_default(writer)?,
//         Descriptor::Reserved() => {
//             unimplemented!();
//         }
//     };
//     Ok(())
// }

impl Encode for u8 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::UBYTE;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_u8(self)?;
        Ok(())
    }
}

impl Encode for i8 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::BYTE;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_i8(self)?;
        Ok(())
    }
}

impl Encode for u16 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::USHORT;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_u16(self)?;
        Ok(())
    }
}

impl Encode for i16 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::SHORT;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_i16(self)?;
        Ok(())
    }
}

impl Encode for u32 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::UINT;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::UINT => writer.write_u32(self)?,
            FormatCode::UINT_0 => {
                debug_assert_eq!(self, 0);
            }
            FormatCode::SMALL_UINT => {
                debug_assert!(self <= u8::MAX as u32);
                writer.write_u8(self as u8)?;
            }
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl Encode for i32 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::INT;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::INT => writer.write_i32(self)?,
            FormatCode::SMALL_INT => {
                debug_assert!(i8::try_from(self).is_ok());
                writer.write_i8(self as i8)?;
            }
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl Encode for u64 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::ULONG;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::ULONG => writer.write_u64(self)?,
            FormatCode::ULONG_0 => {
                debug_assert_eq!(self, 0)
            }
            FormatCode::SMALL_ULONG => {
                debug_assert!(self <= u8::MAX as u64);
                writer.write_u8(self as u8)?
            }
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl Encode for i64 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::LONG;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::LONG => writer.write_i64(self)?,
            FormatCode::SMALL_LONG => {
                debug_assert!(i8::try_from(self).is_ok());
                writer.write_i8(self as i8)?
            }
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl Encode for f32 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::FLOAT;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_f32(self)?;
        Ok(())
    }
}

impl Encode for f64 {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::DOUBLE;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_f64(self)?;
        Ok(())
    }
}

impl Encode for bool {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::BOOLEAN;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::BOOLEAN => {
                writer.write_u8(self as u8)?;
            }
            FormatCode::BOOLEAN_FALSE => {}
            FormatCode::BOOLEAN_TRUE => {}
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl Encode for char {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::CHAR;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_u32(self as u32)?;
        Ok(())
    }
}

impl Encode for Uuid {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::UUID;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_slice(&self.0)?;
        Ok(())
    }
}

impl Encode for Ts {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::TIMESTAMP;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        writer.write_u64(self.0)?;
        Ok(())
    }
}

impl<'a> Encode for &'a str {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::STRING32_UTF8;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::STRING8_UTF8 => writer.write_variable_8(self.as_bytes())?,
            FormatCode::STRING32_UTF8 => writer.write_variable_32(self.as_bytes())?,
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}
impl<'a> Encode for Symbol<'a> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::SYMBOL32;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::SYMBOL8 => writer.write_variable_8(self.0)?,
            FormatCode::SYMBOL32 => writer.write_variable_32(self.0)?,
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl<'a> Encode for Binary<'a> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::BINARY32;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match format_code {
            FormatCode::BINARY8 => writer.write_variable_8(self.0)?,
            FormatCode::BINARY32 => writer.write_variable_32(self.0)?,
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl<'a, T: Multiple + Type<'a>> Encode for Array<'a, T> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::ARRAY32;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        let fwrite = move |writer: &mut Writer| {
            writer.write_constructor(Constructor {
                descriptor: T::DESCRIPTOR,
                format_code: T::ENCODE_DEFAULT_FORMAT_CODE,
            })?;
            let mut count: usize = 0;
            for item in self {
                let item = item?;
                item.encode_data(T::ENCODE_DEFAULT_FORMAT_CODE, writer)?;
                count += 1;
            }
            io::Result::Ok(count)
        };
        match format_code {
            FormatCode::ARRAY8 => {
                writer.write_items_8(fwrite)?;
            }
            FormatCode::ARRAY32 => {
                writer.write_items_32(fwrite)?;
            }
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl<'a, K: Type<'a>, V: Type<'a>> Encode for Map<'a, K, V> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::MAP32;

    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        let fwrite = move |writer: &mut Writer| {
            let mut count: usize = 0;
            for item in self {
                let (k, v) = item?;
                k.encode_default(writer)?;
                v.encode_default(writer)?;
                count += 2;
            }
            io::Result::Ok(count)
        };
        match format_code {
            FormatCode::MAP8 => {
                writer.write_items_8(fwrite)?;
            }
            FormatCode::MAP32 => {
                writer.write_items_32(fwrite)?;
            }
            code => panic!("invalid format code {code:?}"),
        }
        Ok(())
    }
}

impl Encode for () {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = FormatCode::NULL;

    fn encode_data(self, format_code: FormatCode, _writer: &mut Writer) -> io::Result<()> {
        debug_assert_eq!(format_code, Self::ENCODE_DEFAULT_FORMAT_CODE);
        Ok(())
    }
}

impl<'a, T: Type<'a>> Encode for Option<T> {
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = T::ENCODE_DEFAULT_FORMAT_CODE;
    const DESCRIPTOR: Option<Descriptor<'static>> = T::DESCRIPTOR;
    fn encode_data(self, format_code: FormatCode, writer: &mut Writer) -> io::Result<()> {
        match self {
            Some(v) => v.encode_data(format_code, writer),
            None => ().encode_data(FormatCode::NULL, writer),
        }
    }

    fn encode(self, constructor: Constructor, writer: &mut Writer) -> io::Result<()> {
        match self {
            Some(v) => v.encode(constructor, writer),
            None => ().encode_default(writer),
        }
    }

    fn encode_default(self, writer: &mut Writer) -> io::Result<()> {
        match self {
            Some(v) => v.encode_default(writer),
            None => ().encode_default(writer),
        }
    }
}

impl<'a> Encode for Value<'a> {
    const DESCRIPTOR: Option<Descriptor<'static>> = unreachable!();
    const ENCODE_DEFAULT_FORMAT_CODE: FormatCode = unreachable!();
    fn encode_data(self, _format_code: FormatCode, _writer: &mut Writer) -> io::Result<()> {
        unreachable!("don't encode value's data directly, use encode instead")
    }
    fn encode(self, _constructor: Constructor, writer: &mut Writer) -> io::Result<()> {
        self.encode_default(writer)
    }
    fn encode_default(self, writer: &mut Writer) -> io::Result<()> {
        writer.write_constructor(self.constructor)?;
        writer.write_slice(self.data.into_inner())?;
        Ok(())
    }
}
