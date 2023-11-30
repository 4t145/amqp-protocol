pub struct Open<'b> {
    pub container_id: &'a str,
    pub hostname: &'a str,
    pub max_frame_size: u32,
    pub channel_max: u16,
    pub idle_timeout: u32,
    pub outgoing_locales: Vec<&'a str>,
    pub incoming_locales: Vec<&'a str>,
    pub offered_capabilities: Vec<&'a str>,
    pub desired_capabilities: Vec<&'a str>,
    pub properties: Vec<(&'a str, &'a str)>,
}

