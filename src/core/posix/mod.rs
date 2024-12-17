use std::time::{SystemTime, UNIX_EPOCH};

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

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for Posix {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
