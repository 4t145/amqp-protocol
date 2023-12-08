use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{amqp_type, transport::definitions::Fields};
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Open<'a> {
    /// the id of the source container
    pub(crate) container_id: &'a str,
    /// the name of the target host
    pub(crate) hostname: Option<&'a str>,
    /// proposed maximum frame size
    pub(crate) max_frame_size: u32,
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: Option<u32>,
    pub(crate) outgoing_locales: Vec<&'a str>,
    pub(crate) incoming_locales: Vec<&'a str>,
    pub(crate) offered_capabilities: Vec<&'a str>,
    pub(crate) desired_capabilities: Vec<&'a str>,
    pub(crate) properties: Fields<'a>,
}

impl<'a> Default for Open<'a> {
    fn default() -> Self {
        Self {
            container_id: Default::default(),
            hostname: None,
            max_frame_size: u32::MAX,
            channel_max: u16::MAX,
            idle_timeout: Default::default(),
            outgoing_locales: Default::default(),
            incoming_locales: Default::default(),
            offered_capabilities: Default::default(),
            desired_capabilities: Default::default(),
            properties: Default::default(),
        }
    }
}

amqp_type! {Open<'_> = 0x00000000:0x00000010}
