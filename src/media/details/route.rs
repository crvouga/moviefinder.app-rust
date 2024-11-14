use serde::{Deserialize, Serialize};

use crate::media::media_id::MediaId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    IndexLoad { media_id: MediaId },
    Index { media_id: MediaId },
}
