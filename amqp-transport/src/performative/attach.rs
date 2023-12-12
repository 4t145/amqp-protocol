// use crate::amqp_type;

// pub struct Attach {

// }

// derive_descriptor! {Attach = 0x00000000:0x00000012}

use std::collections::HashMap;

use amqp_types::{Symbol, Types, Value};

use crate::definitions::*;

pub trait Source: Types {}

pub trait Target: Types {}

pub struct Attach<S, T>
where
    S: Source,
    T: Target,
{
    pub(crate) name: String,
    pub(crate) handle: u32,
    pub(crate) role: Role,
    pub(crate) snd_settle_mode: SenderSettleMode,
    pub(crate) rcv_settle_mode: ReceiverSettleMode,
    pub(crate) source: Option<S>,
    pub(crate) target: Option<T>,
    pub(crate) unsettled: Option<HashMap<Value, Value>>,
    pub(crate) incomplete_unsettled: Option<bool>,
    pub(crate) initial_delivery_count: Option<u32>,
    pub(crate) max_message_size: Option<u64>,
    pub(crate) offered_capabilities: Option<Vec<Symbol>>,
    pub(crate) desired_capabilities: Option<Vec<Symbol>>,
    pub(crate) properties: Option<Fields>,
}
