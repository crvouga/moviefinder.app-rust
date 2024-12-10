use serde::{Deserialize, Serialize};

use crate::ui::route::Routable;

use super::{edit_profile, login_with_sms, logout};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    AccountScreen,
    LoginWithSms(login_with_sms::route::Route),
    Logout(logout::route::Route),
    EditProfile(edit_profile::route::Route),
}

impl Routable for edit_profile::route::Route {
    fn url(&self) -> String {
        Route::EditProfile(self.clone()).url()
    }
}

impl Routable for login_with_sms::route::Route {
    fn url(&self) -> String {
        Route::LoginWithSms(self.clone()).url()
    }
}

impl Routable for logout::route::Route {
    fn url(&self) -> String {
        Route::Logout(self.clone()).url()
    }
}
