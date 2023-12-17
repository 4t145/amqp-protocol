use std::time::{Instant, SystemTime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ts(pub(crate) u64);

impl Ts {
    pub fn into_system_time(self) -> SystemTime {
        self.into()
    }
}

impl From<Ts> for SystemTime {
    fn from(val: Ts) -> Self {
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(val.0)
    }
}

impl From<u64> for Ts {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Ts> for u64 {
    fn from(value: Ts) -> Self {
        value.0
    }
}
