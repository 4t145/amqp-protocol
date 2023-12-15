pub use array::*;
pub use binary::*;
pub use list::*;
pub use map::*;
pub use symbol::*;

mod array;
mod binary;
mod list;
mod map;
mod symbol;

#[derive(Debug, Clone)]
pub enum Primitive<'frame> {
    Null,
    Boolean(bool),
    UByte(u8),
    UShort(u16),
    UInt(u32),
    ULong(u64),
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Decimal32(),
    Decimal64(),
    Decimal128(),
    Char(char),
    Timestamp(u64),
    Uuid([u8; 16]),
    String(&'frame str),
    Binary(Binary<'frame>),
    Symbol(Symbol<'frame>),
    List(ListIter<'frame>),
    Map(MapIter<'frame>),
    Array(ArrayIter<'frame>),
}
