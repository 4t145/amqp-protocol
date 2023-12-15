#[derive(Debug, Clone)]
pub struct Binary<'frame>(pub(crate) &'frame [u8]);