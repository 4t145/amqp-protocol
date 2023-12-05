use crate::{derive_descriptor, transport::definitions::*, types::value::Symbol};

// descriptor name="amqp:begin:list" code="0x00000000:0x00000011"
pub struct Begin<'a> {
    remote_channel: u16,
    next_outgoing_i: TransferNumber,
    incoming_window: u32,
    outgoing_window: u32,
    handle_max: Handle,
    offered_capabilities: Symbol<'a>,
    desired_capabilities: Symbol<'a>,
    properties: Fields<'a>,
}

derive_descriptor! {Begin<'_> = 0x00000000:0x00000011}
