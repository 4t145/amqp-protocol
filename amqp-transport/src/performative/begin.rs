

// descriptor name="amqp:begin:list" code="0x00000000:0x00000011"

use amqp_types::{Symbol, Types};

use crate::definitions::{TransferNumber, Handle, Fields};

#[derive(Debug, Types)]
#[amqp(descriptor = 0x_00000000_00000011)]
pub struct Begin {
    remote_channel: u16,
    next_outgoing_i: TransferNumber,
    incoming_window: u32,
    outgoing_window: u32,
    handle_max: Handle,
    offered_capabilities: Symbol,
    desired_capabilities: Symbol,
    properties: Fields,
}

