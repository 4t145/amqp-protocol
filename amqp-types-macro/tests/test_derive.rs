use amqp_types::{primitive::Array, Primitive, Type};

#[derive(Debug, Type, Default)]
#[amqp(descriptor = 0x_00000003_00000002)]
pub struct Open<'amqp> {
    /// the id of the source container
    pub(crate) container_id: &'amqp str,
    /// the name of the target host
    pub(crate) hostname: Option<&'amqp str>,
    /// proposed maximum frame size
    pub(crate) max_frame_size: u32,
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: Option<u32>,
    pub(crate) outgoing_locales: Array<'amqp, &'amqp str>,
    pub(crate) incoming_locales: Array<'amqp, &'amqp str>,
    pub(crate) offered_capabilities: Array<'amqp, &'amqp str>,
    pub(crate) desired_capabilities: Array<'amqp, &'amqp str>,
    // pub(crate) properties: Fields<'a>,
}

#[test]
fn test() {
    let open = Open::default();
    // let value = dbg!(open.as_value());
    // let primitive = dbg!(value.construct().unwrap());
    // dbg!(Open::from_primitive(primitive).unwrap());
}
