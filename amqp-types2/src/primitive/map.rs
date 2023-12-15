use crate::constructor::Constructor;
#[derive(Debug, Clone)]
pub struct Map<'frame> {
    constructor: Constructor<'frame>,
    count: usize,
    data: &'frame [u8]
}