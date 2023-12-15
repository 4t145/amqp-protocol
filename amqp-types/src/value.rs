use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    codec::enc::Encode,
    constructor::{self, Constructor},
    primitives::Primitive,
    FormatCode,
};
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct Value {
//     pub constructor: Constructor,
//     pub data: Bytes,
// }

// impl Value {
//     pub fn new(constructor: impl Into<Constructor>, data: impl Into<Bytes>) -> Self {
//         Value {
//             constructor: constructor.into(),
//             data: data.into(),
//         }
//     }
//     pub fn construct(&self) -> Option<Primitive> {
//         let mut data = self.data.clone();
//         self.constructor.construct(&mut data)
//     }
//     pub fn new_array<T: Encode>(
//         constructor: impl Into<Constructor>,
//         iter: impl IntoIterator<Item = T>,
//     ) -> Self {
//         let item_constructor: Constructor = constructor.into();
//         let mut data = BytesMut::new();
//         let mut size = 0;
//         let mut count = 0;
//         data.put_u32(size);
//         data.put_u32(count);
//         item_constructor.encode(&mut data);
//         for item in iter {
//             count += 1;
//             item.encode(&mut data);
//         }
//         size = data.len() as u32;
//         data[0..4].copy_from_slice(&size.to_be_bytes());
//         data[4..8].copy_from_slice(&count.to_be_bytes());
//         let constructor = Constructor::new(FormatCode::ARRAY32);
//         Value {
//             constructor,
//             data: data.freeze(),
//         }
//     }
// }

pub struct Value<'frame> {
    constructor: Constructor<'frame>,
    data: Data<'frame>
}

pub struct Data<'frame> {
    bytes: &'frame [u8]
}