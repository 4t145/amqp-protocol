use std::collections::HashMap;

use amqp_types::{Symbol, Types, Value};

#[derive(Debug, Clone, Types)]
#[amqp(restrict(source = bool))]
pub enum Role {
    #[amqp(choice = false)]
    Sender,
    #[amqp(choice = true)]
    Receiver,
}


#[derive(Debug, Clone, Default, Types)]
pub struct Fields(pub HashMap<Symbol, Value>);

#[derive(Debug, Clone, Copy, Default, Types, PartialEq, Eq, Hash)]
pub struct Handle(pub u32);

#[derive(Debug, Clone, Copy, Default, Types)]
pub struct SequenceNo(pub u32);

#[derive(Debug, Clone, Copy, Default, Types)]
pub struct TransferNumber(pub u32);





#[test]
fn test() {
    let x = Handle::default();
    let value = x.as_value();
    let y = Handle::from_value(value).unwrap();
    assert_eq!(x, y);
    let role = Role::Receiver;
    let value = role.as_value();
    dbg!(&value);
    let role2 = Role::from_value(value).unwrap();
    dbg!(role2);
}

pub enum TestEnum {
    A,
    B,
    C,
}

impl Types for TestEnum {
    type Source = u8;
    const FORMAT_CODE: amqp_types::FormatCode = Self::Source::FORMAT_CODE;

    fn as_data(&self) -> bytes::Bytes {
        self.unrestrict().as_data()
    }

    fn from_primitive(value: amqp_types::Primitive) -> Option<Self::Source> {
        Self::Source::from_primitive(value)
    }

    fn restrict(source: Self::Source) -> Option<Self> {
        match source {
            0 => Some(Self::A),
            1 => Some(Self::B),
            2 => Some(Self::C),
            _ => None,
        }
    }

    fn unrestrict(&self) -> &Self::Source {
        
        match self {
            Self::A => &0,
            Self::B => &1,
            Self::C => &2,
        }
    }
}
