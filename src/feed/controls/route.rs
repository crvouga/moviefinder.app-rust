use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize)]
pub enum Route {
    IndexLoad,
    Index,
    ClickedSave,
    InputtedSearch,
}
