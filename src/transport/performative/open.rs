use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{derive_descriptor, transport::definitions::Fields};
#[derive(Debug, Serialize, Deserialize)]
pub struct Open<'a> {
    /// the id of the source container
    pub(crate) container_id: &'a str,
    /// the name of the target host
    pub(crate) hostname: &'a str,
    /// proposed maximum frame size
    pub(crate) max_frame_size: u32,
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: u32,
    pub(crate) outgoing_locales: Vec<&'a str>,
    pub(crate) incoming_locales: Vec<&'a str>,
    pub(crate) offered_capabilities: Vec<&'a str>,
    pub(crate) desired_capabilities: Vec<&'a str>,
    pub(crate) properties: Fields<'a>,
}

derive_descriptor! {Open<'_> = 0x00000000:0x00000010}
