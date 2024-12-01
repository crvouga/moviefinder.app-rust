use serde::{Deserialize, Serialize};

use super::login_with_sms;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Screen,
    LoginWithSms(login_with_sms::route::Route),
}
