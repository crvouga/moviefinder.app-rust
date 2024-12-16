use serde::{Deserialize, Serialize};

use crate::ui::route::AppRoute;

use super::{edit_profile, login, login_with_sms, logout};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    AccountScreen,
    LoginWithSms(login_with_sms::route::Route),
    Logout(logout::route::Route),
    Login(login::route::Route),
    EditProfile(edit_profile::route::Route),
}

impl AppRoute for edit_profile::route::Route {
    fn url(&self) -> String {
        Route::EditProfile(self.clone()).url()
    }
}

impl AppRoute for login_with_sms::route::Route {
    fn url(&self) -> String {
        Route::LoginWithSms(self.clone()).url()
    }
}

impl AppRoute for logout::route::Route {
    fn url(&self) -> String {
        Route::Logout(self.clone()).url()
    }
}

impl AppRoute for login::route::Route {
    fn url(&self) -> String {
        Route::Login(self.clone()).url()
    }
}
