use std::{
    any::Any,
    collections::{HashMap, HashSet},
    ptr,
};

use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    codec::enc::Encode,
    codes::FormatCode,
    constructor::Constructor,
    descriptor::Descriptor,
    primitives::{Binary, Symbol},
    value::Value,
    Primitive,
};

pub trait Types: Sized {
    const DESCRIPTOR: Option<Descriptor> = None;
    const FORMAT_CODE: FormatCode;
    #[inline]
    fn default_constructor() -> Constructor {
        Constructor {
            descriptor: Self::DESCRIPTOR,
            format_code: Self::FORMAT_CODE,
        }
    }
    #[inline]
    fn constructor(&self) -> Constructor {
        Self::default_constructor()
    }
    fn as_value(&self) -> Value {
        Value {
            constructor: self.constructor(),
            data: self.as_data(),
        }
    }
    fn as_data(&self) -> Bytes;
    fn from_primitive(value: Primitive) -> Option<Self>;
}


impl Types for Value {
    const FORMAT_CODE: FormatCode = FormatCode::Primitive(0x00);
    fn constructor(&self) -> Constructor {
        self.constructor
    }
    fn as_data(&self) -> Bytes {
        let mut data = BytesMut::new();
        self.constructor().encode(&mut data);
        data.extend(self.data.clone());
        data.into()
    }
    fn from_primitive(value: Primitive) -> Option<Self> {
        Some(Value {
            constructor: Constructor {
                descriptor: None,
                format_code: FormatCode::DESCRIBED,
            },
            data: value.as_data(),
        })
    }
}
// generated by copilot
macro_rules! derive_primitives {
    ($($ty:ty, $code:ident $prim:ident);* $(;)?) => {
        $(
            impl Types for $ty {
                const FORMAT_CODE: FormatCode = FormatCode::$code;
                fn as_data(&self) -> Bytes {
                    Bytes::from(self.to_be_bytes().to_vec())
                }
                fn from_primitive(value: Primitive) -> Option<Self> {
                    if let Primitive::$prim(v) = value {
                        Some(v)
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

derive_primitives! {
    i8, BYTE Byte;
    i16, SHORT Short;
    i32, INT Int ;
    i64, LONG Long ;
    u8, UBYTE UByte ;
    u16, USHORT UShort ;
    u32, UINT UInt ;
    u64, ULONG ULong ;
    f32, FLOAT Float ;
    f64, DOUBLE Double ;
}

impl Types for bool {
    const FORMAT_CODE: FormatCode = FormatCode::BOOLEAN;
    fn as_data(&self) -> Bytes {
        if *self {
            Bytes::from_static(&[1])
        } else {
            Bytes::from_static(&[0])
        }
    }
    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Boolean(v) = value {
            Some(v)
        } else {
            None
        }
    }
}

impl Types for char {
    const FORMAT_CODE: FormatCode = FormatCode::CHAR;
    fn as_data(&self) -> Bytes {
        Bytes::from((*self as u32).to_be_bytes().to_vec())
    }
    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Char(c) = value {
            Some(c)
        } else {
            None
        }
    }
}

impl Types for String {
    const FORMAT_CODE: FormatCode = FormatCode::STRING32_UTF8;
    fn as_data(&self) -> Bytes {
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        data.put_u32(size);
        data.put(self.as_bytes());
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }
    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::String(c) = value {
            Some(c.as_str().ok()?.to_string())
        } else {
            None
        }
    }
}

impl Types for Binary {
    const FORMAT_CODE: FormatCode = FormatCode::BINARY32;
    fn as_data(&self) -> Bytes {
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        data.put_u32(size);
        data.put(self.0.as_ref());
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }

    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Binary(b) = value {
            Some(b)
        } else {
            None
        }
    }
}

impl Types for Symbol {
    const FORMAT_CODE: FormatCode = FormatCode::SYMBOL32;
    fn as_data(&self) -> Bytes {
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        data.put_u32(size);
        data.put(self.0.as_ref());
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }

    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Symbol(s) = value {
            Some(s)
        } else {
            None
        }
    }
}

impl<T: Types + Any> Types for Vec<T> {
    const FORMAT_CODE: FormatCode = FormatCode::ARRAY32;
    fn as_data(&self) -> Bytes {
        let count = self.len() as u32;
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        let item_constructor = T::default_constructor();
        data.put_u32(size);
        data.put_u32(count);
        item_constructor.encode(&mut data);
        for item in self {
            data.extend(item.as_data());
        }
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }

    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Array(a) = value {
            let mut v = Vec::with_capacity(a.count);
            for item in a {
                v.push(T::from_primitive(item)?)
            }
            Some(v)
        } else {
            None
        }
    }
}

impl<const N: usize, T: Types> Types for [T; N] {
    const FORMAT_CODE: FormatCode = FormatCode::ARRAY32;
    fn as_data(&self) -> Bytes {
        let count = self.len() as u32;
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        let item_constructor = T::default_constructor();
        data.put_u32(size);
        data.put_u32(count);
        item_constructor.encode(&mut data);
        for item in self {
            data.extend(item.as_data());
        }
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }

    fn from_primitive(value: Primitive) -> Option<Self> {
        use std::mem::MaybeUninit;
        if let Primitive::Array(mut a) = value {
            if a.count != N {
                return None;
            }

            unsafe {
                let mut array: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();

                for elem in &mut array {
                    ptr::write(elem.as_mut_ptr(), T::from_primitive(a.next()?)?);
                }

                // this should be safe since each element have been initialized
                let array: [T; N] =
                    ptr::read(&array as *const [MaybeUninit<T>; N] as *const [T; N]);

                Some(array)
            }
        } else {
            None
        }
    }
}

impl<T: Types + Eq + std::hash::Hash> Types for HashSet<T> {
    const FORMAT_CODE: FormatCode = FormatCode::ARRAY32;
    fn as_data(&self) -> Bytes {
        let count = self.len() as u32;
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        let item_constructor = T::default_constructor();
        data.put_u32(size);
        data.put_u32(count);
        item_constructor.encode(&mut data);
        for item in self {
            data.extend(item.as_data());
        }
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }

    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Array(a) = value {
            let mut s = HashSet::with_capacity(a.count);
            for item in a {
                s.insert(T::from_primitive(item)?);
            }
            Some(s)
        } else {
            None
        }
    }
}

impl<K: Types + Eq + std::hash::Hash, V: Types + Eq> Types for HashMap<K, V> {
    const FORMAT_CODE: FormatCode = FormatCode::ARRAY32;
    fn as_data(&self) -> Bytes {
        let count = (self.len() as u32) * 2;
        let mut size: u32 = 0;
        let mut data = BytesMut::new();
        data.put_u32(size);
        data.put_u32(count);
        for (k, v) in self {
            k.as_value().encode(&mut data);
            v.as_value().encode(&mut data);
        }
        size = data.len() as u32 - 4;
        data[0..4].copy_from_slice(&size.to_be_bytes());
        data.into()
    }
    fn from_primitive(value: Primitive) -> Option<Self> {
        if let Primitive::Map(m) = value {
            let mut map = HashMap::with_capacity(m.count);
            for (k, v) in m {
                map.insert(
                    K::from_primitive(k.construct()?)?,
                    V::from_primitive(v.construct()?)?,
                );
            }
            Some(map)
        } else {
            None
        }
    }
}

impl<T: Types> Types for Option<T> {
    const FORMAT_CODE: FormatCode = T::FORMAT_CODE;
    const DESCRIPTOR: Option<Descriptor> = T::DESCRIPTOR;
    fn constructor(&self) -> Constructor {
        match self {
            Some(v) => v.constructor(),
            None => Constructor {
                descriptor: Self::DESCRIPTOR,
                format_code: FormatCode::NULL,
            },
        }
    }
    fn as_data(&self) -> Bytes {
        match self {
            Some(v) => v.as_data(),
            None => Bytes::new(),
        }
    }
    fn from_primitive(value: Primitive) -> Option<Self> {
        match value {
            Primitive::Null => Some(None),
            _ => Some(Some(T::from_primitive(value)?)),
        }
    }
}

macro_rules! impl_tuple_types {
    ($($T:ident),*) => {
        impl<$($T: Types),*> Types for ($($T,)*) {
            const FORMAT_CODE: FormatCode = FormatCode::LIST32;
            fn as_data(&self) -> Bytes {
                #[allow(non_snake_case)]
                let ($($T,)*) = self;
                let mut count = 0;
                let mut size: u32 = 0;
                let mut data = BytesMut::new();
                data.put_u32(size);
                data.put_u32(count);
                $(
                    count += 1;
                    data.extend($T.as_data());
                )*
                size = data.len() as u32 - 4;
                data[0..4].copy_from_slice(&size.to_be_bytes());
                data[4..8].copy_from_slice(&count.to_be_bytes());
                data.into()
            }
            fn from_primitive(value: Primitive) -> Option<Self> {
                match value {
                    Primitive::List(l) => {
                        let mut iter = l.into_iter();
                        Some((
                            $(
                                $T::from_primitive(iter.next()?.construct()?)?,
                            )*
                        ))
                    },
                    _ => None,
                }
            }
        }
    };
}

macro_rules! impl_tuples {
    () => {};
    ($T:ident $(,$Ts:ident)*) => {
        impl_tuple_types!($T $(,$Ts)*);
        impl_tuples!($($Ts),*);
    };
}

impl_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
