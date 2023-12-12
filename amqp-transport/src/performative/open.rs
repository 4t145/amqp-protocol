use std::collections::HashMap;

use amqp_types::{Symbol, Types};

use crate::definitions::{Fields, IetfLanguageTag};

#[derive(Debug, Types)]
#[amqp(descriptor = 0x00000000:0x00000011)]
pub struct Open {
    /// the id of the source container
    pub(crate) container_id: String,
    /// the name of the target host
    pub(crate) hostname: Option<String>,
    /// proposed maximum frame size
    #[amqp(default = u32::MAX)]
    pub(crate) max_frame_size: u32,
    #[amqp(default = u16::MAX)]
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: Option<u32>,
    pub(crate) outgoing_locales: Vec<IetfLanguageTag>,
    pub(crate) incoming_locales: Vec<IetfLanguageTag>,
    pub(crate) offered_capabilities: Vec<Symbol>,
    pub(crate) desired_capabilities: Vec<Symbol>,
    pub(crate) properties: Option<Fields>,
}

impl Default for Open {
    fn default() -> Self {
        Self {
            container_id: String::new(),
            hostname: None,
            max_frame_size: u32::MAX,
            channel_max: u16::MAX,
            idle_timeout: None,
            outgoing_locales: Vec::new(),
            incoming_locales: Vec::new(),
            offered_capabilities: Vec::new(),
            desired_capabilities: Vec::new(),
            properties: None,
        }
    }
}
