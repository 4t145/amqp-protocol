use bytes::Bytes;
use serde::Deserialize;

use crate::{
    codes::FormatCode,
    data::{Data, DataView},
    descriptor::Descriptor,
    value::Value,
};

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

    pub fn construct<T: DataView>(&self, data: &mut Data) -> Option<T> {
        if T::accept(self) {
            T::view(data)
        } else {
            None
        }
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
