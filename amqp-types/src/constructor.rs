use bytes::Bytes;

use crate::{
    codec::{BytesExt, Decode},
    codes::FormatCode,
    descriptor::Descriptor,
    primitives::{AmqpArray, AmqpList, AmqpMap, AmqpString, Binary, Primitive, Symbol},
};

#[derive(Debug, Clone)]
pub struct Constructor {
    pub descriptor: Option<Descriptor>,
    pub format_code: FormatCode,
}

impl Constructor {
    pub fn new(format_code: FormatCode) -> Self {
        Constructor {
            descriptor: None,
            format_code,
        }
    }

    pub fn described(descriptor: Descriptor, format_code: FormatCode) -> Self {
        Constructor {
            descriptor: Some(descriptor),
            format_code,
        }
    }

    pub fn is_described(&self) -> bool {
        self.descriptor.is_some()
    }

    pub fn construct(&self, data: &mut Bytes) -> Option<Primitive> {
        let value = match self.format_code {
            FormatCode::NULL => Primitive::Null,
            FormatCode::BOOLEAN_TRUE => Primitive::Boolean(true),
            FormatCode::BOOLEAN_FALSE => Primitive::Boolean(false),
            FormatCode::UINT_0 => Primitive::UInt(0),
            FormatCode::ULONG_0 => Primitive::ULong(0),
            FormatCode::LIST0 => Primitive::List(AmqpList::default()),
            FormatCode::BOOLEAN => Primitive::Boolean(Decode::decode(data)?),
            FormatCode::UBYTE => Primitive::UByte(Decode::decode(data)?),
            FormatCode::BYTE => Primitive::Byte(Decode::decode(data)?),
            FormatCode::SMALL_UINT => Primitive::UInt(u8::decode(data)? as u32),
            FormatCode::SMALL_ULONG => Primitive::ULong(u8::decode(data)? as u64),
            FormatCode::SMALL_INT => Primitive::Int(i8::decode(data)? as i32),
            FormatCode::SMALL_LONG => Primitive::Long(i8::decode(data)? as i64),
            FormatCode::USHORT => Primitive::UShort(Decode::decode(data)?),
            FormatCode::SHORT => Primitive::UShort(Decode::decode(data)?),
            FormatCode::UINT => Primitive::UInt(Decode::decode(data)?),
            FormatCode::INT => Primitive::Int(Decode::decode(data)?),
            FormatCode::FLOAT => Primitive::Float(Decode::decode(data)?),
            FormatCode::CHAR => {
                todo!()
            }
            FormatCode::DECIMAL32 => {
                todo!()
            }
            FormatCode::ULONG => Primitive::ULong(Decode::decode(data)?),
            FormatCode::LONG => Primitive::Long(Decode::decode(data)?),
            FormatCode::DOUBLE => Primitive::Double(Decode::decode(data)?),
            FormatCode::DECIMAL64 => {
                todo!()
            }
            FormatCode::DECIMAL128 => {
                todo!()
            }
            FormatCode::TIMESTAMP => {
                todo!()
            }
            FormatCode::UUID => Primitive::Uuid(Decode::decode(data)?),
            FormatCode::BINARY8 => {
                let size = u8::decode(data)? as usize;
                Primitive::Binary(Binary(data.try_eat(size)?))
            }
            FormatCode::BINARY32 => {
                let size = u32::decode(data)? as usize;
                Primitive::Binary(Binary(data.try_eat(size)?))
            }
            FormatCode::STRING8_UTF8 => {
                let size = u32::decode(data)? as usize;
                Primitive::String(AmqpString(data.try_eat(size)?))
            }
            FormatCode::STRING32_UTF8 => {
                let size = u32::decode(data)? as usize;
                Primitive::String(AmqpString(data.try_eat(size)?))
            }
            FormatCode::SYMBOL8 => {
                let size = u8::decode(data)? as usize;
                Primitive::Symbol(Symbol(data.try_eat(size)?))
            }
            FormatCode::SYMBOL32 => {
                let size = u32::decode(data)? as usize;
                Primitive::Symbol(Symbol(data.try_eat(size)?))
            }
            FormatCode::LIST8 => {
                let size = u8::decode(data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u8::decode(&mut data)? as usize;
                Primitive::List(AmqpList { count, data })
            }
            FormatCode::LIST32 => {
                let size = u32::decode(data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u32::decode(&mut data)? as usize;
                Primitive::List(AmqpList { count, data })
            }
            FormatCode::MAP8 => {
                let size = u8::decode(data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u8::decode(&mut data)? as usize;
                Primitive::Map(AmqpMap {
                    count: count / 2,
                    data,
                })
            }
            FormatCode::MAP32 => {
                let size = u32::decode(data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u32::decode(&mut data)? as usize;
                Primitive::Map(AmqpMap {
                    count: count / 2,
                    data,
                })
            }
            FormatCode::ARRAY8 => {
                let size = u8::decode(data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u8::decode(&mut data)? as usize;
                Primitive::List(AmqpList { count, data })
            }
            FormatCode::ARRAY32 => {
                let size = u32::decode(data)? as usize;
                let mut data = data.try_eat(size)?;
                let count = u32::decode(&mut data)? as usize;
                let constructor = Constructor::decode(&mut data)?;
                Primitive::Array(AmqpArray {
                    constructor,
                    count,
                    data,
                })
            }
            FormatCode::Primitive(_) => {
                return None;
            }
            FormatCode::Ext(_, _) => {
                return None;
            }
        };
        Some(value)
    }
}

//        let byte = u8::decode(buffer)?;
// match byte {
//     0x00 => {
//         let descriptor = Descriptor::view(buffer)?;
//         let constructor = Constructor::view(buffer)?;
//         Ok(Constructor::Described {
//             descriptor,
//             constructor: Box::new(constructor),
//         })
//     }
//     code if code & 0x0f != 0x0f => Ok(Constructor::FormatCode(FormatCode::Primitive(code))),
//     code => {
//         let ext = u8::decode(buffer)?;
//         Ok(Constructor::FormatCode(FormatCode::Ext(code, ext)))
//     }
// }
// Array[T] -> Vec<T>
// Map[K, V] -> HashMap<K, V>
