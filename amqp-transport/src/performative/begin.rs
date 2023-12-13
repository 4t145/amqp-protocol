

// descriptor name="amqp:begin:list" code="0x00000000:0x00000011"

use amqp_types::{Symbol, Type};

use crate::definitions::{TransferNumber, Handle, Fields};

#[derive(Debug, Type)]
#[amqp(descriptor = 0x00000000:0x00000011)]
pub struct Begin {
    remote_channel: Option<u16>,
    next_outgoing_i: TransferNumber,
    incoming_window: u32,
    outgoing_window: u32,
    #[amqp(default = Handle(u32::MAX))]
    handle_max: Handle,
    offered_capabilities: Vec<Symbol>,
    desired_capabilities: Vec<Symbol>,
    properties: Option<Fields>,
}

