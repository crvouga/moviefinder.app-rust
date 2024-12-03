use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Drawer,
    ClickedLogout,
    ClickedCancel,
}
