#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol<'frame>(&'frame [u8]);

impl<'frame> Symbol<'frame> {
    pub const fn new(bytes: &'frame [u8]) -> Self {
        Symbol(bytes)
    }
}
