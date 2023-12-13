use amqp_types::{Primitive, Type};

#[derive(Debug, Type, Default)]
#[amqp(descriptor = 0x00000003:0x00000002)]
pub struct Open {
    /// the id of the source container
    pub(crate) container_id: String,
    /// the name of the target host
    pub(crate) hostname: Option<String>,
    /// proposed maximum frame size
    pub(crate) max_frame_size: u32,
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: Option<u32>,
    pub(crate) outgoing_locales: Vec<String>,
    pub(crate) incoming_locales: Vec<String>,
    pub(crate) offered_capabilities: Vec<String>,
    pub(crate) desired_capabilities: Vec<String>,
    // pub(crate) properties: Fields<'a>,
}

#[test]
fn test() {
    let open = Open::default();
    let value = dbg!(open.as_value());
    let primitive = dbg!(value.construct().unwrap());
    dbg!(Open::from_primitive(primitive).unwrap());
}
