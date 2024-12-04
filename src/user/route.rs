use serde::{Deserialize, Serialize};

use crate::ui::to_url::ToURL;

use super::{login_with_sms, logout};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    Screen,
    LoginWithSms(login_with_sms::route::Route),
    Logout(logout::route::Route),
}

impl ToURL for login_with_sms::route::Route {
    fn to_url(&self) -> String {
        Route::LoginWithSms(self.clone()).to_url()
    }
}

impl ToURL for logout::route::Route {
    fn to_url(&self) -> String {
        Route::Logout(self.clone()).to_url()
    }
}
