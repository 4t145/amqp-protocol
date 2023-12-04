use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::{derive_descriptor, transport::definitions::Fields};
#[derive(Debug, Serialize, Deserialize)]
pub struct Open {
    /// the id of the source container
    pub(crate) container_id: String,
    /// the name of the target host
    pub(crate) hostname: String,
    /// proposed maximum frame size
    pub(crate) max_frame_size: u32,
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: u32,
    pub(crate) outgoing_locales: Vec<String>,
    pub(crate) incoming_locales: Vec<String>,
    pub(crate) offered_capabilities: Vec<String>,
    pub(crate) desired_capabilities: Vec<String>,
    pub(crate) properties: Fields,
}

derive_descriptor! {Open = 0x00000000:0x00000010}
