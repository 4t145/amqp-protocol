use std::collections::HashMap;

use crate::types::value::{Symbol, Value};

pub struct Fields(HashMap<Symbol, Value>);
