use amqp_types::{codec::{Encode, Decode, Writer}, primitive::Array, Primitive, Type, Value, types::Type};

#[derive(Debug, Type, Default)]
#[amqp(descriptor = 0x_00000003_00000002)]
pub struct Open<'amqp> {
    pub(crate) container_id: &'amqp str,
    pub(crate) hostname: Option<&'amqp str>,
    pub(crate) max_frame_size: u32,
    pub(crate) channel_max: u16,
    pub(crate) idle_timeout: Option<u32>,
    pub(crate) outgoing_locales: Array<'amqp, &'amqp str>,
    pub(crate) incoming_locales: Array<'amqp, &'amqp str>,
    pub(crate) offered_capabilities: Array<'amqp, &'amqp str>,
    pub(crate) desired_capabilities: Array<'amqp, &'amqp str>,
}

#[test]
fn test() {
    let open = Open {
        container_id: "test",
        incoming_locales: Array::new_write(["hello", "amqp"]),
        ..Default::default()
    };
    let mut buffer = vec![0; 128];
    let mut writer = Writer::new(&mut buffer);
    writer.write_amqp_value(open).unwrap();
    let hex = buffer.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ");
    println!("{}", hex);
    let value = dbg!(Value::decode(&mut buffer.as_slice()).unwrap());
    dbg!(value.clone().construct().unwrap());
    let new_open = dbg!(Open::try_from_value(value).unwrap());
    let s = new_open.container_id;
    assert_eq!(s, "test");
}
