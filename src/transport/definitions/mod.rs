use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::value::{Symbol, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Fields<'a>(HashMap<Symbol<'a>, Value<'a>>);
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]

pub struct Handle(u32);
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]

pub struct SequenceNo(u32);
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct TransferNumber(u32);
