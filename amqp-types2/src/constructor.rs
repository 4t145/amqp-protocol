use crate::codec::de::{decode_str, Decode, DecodeExt};
use crate::primitive::{ArrayIter, Binary, ListIter, Symbol};
use crate::{
    data::Data, descriptor::Descriptor, error::UNKNOWN_AMQP_TYPE, format_code::FormatCode,
    primitive::Primitive,
};
use std::io;
#[derive(Debug, Clone)]
pub struct Constructor<'frame> {
    pub descriptor: Option<Descriptor<'frame>>,
    pub format_code: FormatCode,
}

impl<'frame> Constructor<'frame> {
    pub fn construct(self, data: Data<'frame>) -> io::Result<Primitive<'frame>> {
        let mut data = data.into_inner();
        let value = match self.format_code {
            FormatCode::NULL => Primitive::Null,
            FormatCode::BOOLEAN_TRUE => Primitive::Boolean(true),
            FormatCode::BOOLEAN_FALSE => Primitive::Boolean(false),
            FormatCode::UINT_0 => Primitive::UInt(0),
            FormatCode::ULONG_0 => Primitive::ULong(0),
            FormatCode::LIST0 => Primitive::List(ListIter::default()),
            FormatCode::BOOLEAN => Primitive::Boolean(Decode::decode(&mut data)?),
            FormatCode::UBYTE => Primitive::UByte(Decode::decode(&mut data)?),
            FormatCode::BYTE => Primitive::Byte(Decode::decode(&mut data)?),
            FormatCode::SMALL_UINT => Primitive::UInt(u8::decode(&mut data)? as u32),
            FormatCode::SMALL_ULONG => Primitive::ULong(u8::decode(&mut data)? as u64),
            FormatCode::SMALL_INT => Primitive::Int(i8::decode(&mut data)? as i32),
            FormatCode::SMALL_LONG => Primitive::Long(i8::decode(&mut data)? as i64),
            FormatCode::USHORT => Primitive::UShort(Decode::decode(&mut data)?),
            FormatCode::SHORT => Primitive::UShort(Decode::decode(&mut data)?),
            FormatCode::UINT => Primitive::UInt(Decode::decode(&mut data)?),
            FormatCode::INT => Primitive::Int(Decode::decode(&mut data)?),
            FormatCode::FLOAT => Primitive::Float(Decode::decode(&mut data)?),
            FormatCode::CHAR => Primitive::Char(Decode::decode(&mut data)?),
            // FormatCode::DECIMAL32 => {
            //     todo!()
            // }
            FormatCode::ULONG => Primitive::ULong(Decode::decode(&mut data)?),
            FormatCode::LONG => Primitive::Long(Decode::decode(&mut data)?),
            FormatCode::DOUBLE => Primitive::Double(Decode::decode(&mut data)?),
            // FormatCode::DECIMAL64 => {
            //     todo!()
            // }
            // FormatCode::DECIMAL128 => {
            //     todo!()
            // }
            // FormatCode::TIMESTAMP => {
            //     todo!()
            // }
            FormatCode::UUID => Primitive::Uuid(Decode::decode(&mut data)?),
            FormatCode::BINARY8 => {
                let size = u8::decode(&mut data)? as usize;
                Primitive::Binary(Binary(data.try_eat(size)?))
            }
            FormatCode::BINARY32 => {
                let size = u32::decode(&mut data)? as usize;
                Primitive::Binary(Binary(data.try_eat(size)?))
            }
            FormatCode::STRING8_UTF8 => {
                let size = u32::decode(&mut data)? as usize;
                decode_str(&mut data, size).map(Primitive::String)?
            }
            FormatCode::STRING32_UTF8 => {
                let size = u32::decode(&mut data)? as usize;
                decode_str(&mut data, size).map(Primitive::String)?
            }
            FormatCode::SYMBOL8 => {
                let size = u8::decode(&mut data)? as usize;
                Primitive::Symbol(Symbol(data.try_eat(size)?))
            }
            FormatCode::SYMBOL32 => {
                let size = u32::decode(&mut data)? as usize;
                Primitive::Symbol(Symbol(data.try_eat(size)?))
            }
            // FormatCode::LIST8 => {
            //     let size = u8::decode(data)? as usize;
            //     let mut data = data.try_eat(size)?;
            //     let count = u8::decode(&mut data)? as usize;
            //     Primitive::List(AmqpList { count, data })
            // }
            // FormatCode::LIST32 => {
            //     let size = u32::decode(data)? as usize;
            //     let mut data = data.try_eat(size)?;
            //     let count = u32::decode(&mut data)? as usize;
            //     Primitive::List(AmqpList { count, data })
            // }
            // FormatCode::MAP8 => {
            //     let size = u8::decode(data)? as usize;
            //     let mut data = data.try_eat(size)?;
            //     let count = u8::decode(&mut data)? as usize;
            //     Primitive::Map(AmqpMap {
            //         count: count / 2,
            //         data,
            //     })
            // }
            // FormatCode::MAP32 => {
            //     let size = u32::decode(data)? as usize;
            //     let mut data = data.try_eat(size)?;
            //     let count = u32::decode(&mut data)? as usize;
            //     Primitive::Map(AmqpMap {
            //         count: count / 2,
            //         data,
            //     })
            // }
            FormatCode::ARRAY8 => {
                let size = u8::decode(&mut data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u8::decode(&mut data)? as usize;
                let constructor = Constructor::decode(&mut data)?;
                Primitive::Array(ArrayIter {
                    constructor,
                    count,
                    items_data: data,
                })
            }
            FormatCode::ARRAY32 => {
                let size = u32::decode(&mut data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u32::decode(&mut data)? as usize;
                let constructor = Constructor::decode(&mut data)?;
                Primitive::Array(ArrayIter {
                    constructor,
                    count,
                    items_data: data,
                })
            }
            FormatCode::Primitive(p) => {
                return Err(io::Error::other(format!("{UNKNOWN_AMQP_TYPE}[{p:02x}]")));
            }
            FormatCode::Ext(c, b) => {
                return Err(io::Error::other(format!(
                    "{UNKNOWN_AMQP_TYPE}[{c:02x}:{b:02x}]"
                )));
            }
        };
        Ok(value)
    }

    pub fn peek_size(&self, data: &[u8]) -> io::Result<usize> {
        self.format_code.peek_size(data)
    }
}
