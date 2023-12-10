use std::collections::HashMap;

use amqp_types::{Symbol, Value, Types};

#[derive(Debug, Clone, Default, Types)]
pub struct Fields(pub HashMap<Symbol, Value>);


#[derive(Debug, Clone, Copy, Default, Types)]
pub struct Handle(pub u32);

#[derive(Debug, Clone, Copy, Default, Types)]
pub struct SequenceNo(pub u32);

#[derive(Debug, Clone, Copy, Default, Types)]
pub struct TransferNumber(pub u32);


#[test]
fn test(){
    let x = Handle::default();
    x.0;
}