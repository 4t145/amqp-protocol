pub enum Primitive {
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
    String(String),
    Symbol(Vec<u8>),
    List(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Array(Vec<Value>),
}

impl Into<Value> for Primitive {
    fn into(self) -> Value {
        Value::Primitive(self)
    }
}

pub struct Described {
    descriptor: Descriptor,
    value: Value,
}

pub enum Descriptor {
    Symbol(Vec<u8>),
    Numeric(u32, u32),
}

pub enum Value {
    Primitive(Primitive),
    Described(Box<Described>),
}
