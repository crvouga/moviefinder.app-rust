use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Ord, PartialOrd, Copy, Hash,
)]
pub struct Posix(i64);

impl Posix {
    pub fn now() -> Self {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let now = since_epoch.as_secs() as i64;
        Self(now)
    }

    #[allow(dead_code)]
    pub fn future(&self, duration: Duration) -> Self {
        let future = self.0 + duration.as_secs() as i64;
        Self(future)
    }

    pub fn diff(&self, other: &Posix) -> Duration {
        let max = self.0.max(other.0);
        let min = self.0.min(other.0);
        Duration::from_secs((max - min) as u64)
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for Posix {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PosixNow {
    #[default]
    SystemTime,
    Static(Posix),
}

impl PosixNow {
    #[allow(dead_code)]
    pub fn now(&self) -> Posix {
        match self {
            Self::Static(posix) => *posix,
            Self::SystemTime => Posix::now(),
        }
    }
}
