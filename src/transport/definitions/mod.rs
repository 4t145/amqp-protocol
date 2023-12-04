use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::types::value::{Symbol, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Fields(HashMap<Symbol, Value>);

pub struct Handle(u32);

pub struct SequenceNo(u32);
pub struct TransferNumber(u32);
