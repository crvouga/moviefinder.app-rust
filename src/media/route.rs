use serde::{Deserialize, Serialize};

use super::details;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Details(details::route::Route),
}
