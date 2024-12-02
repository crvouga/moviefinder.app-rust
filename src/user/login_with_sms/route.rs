use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Route {
    ScreenPhone,
    ScreenCode { phone_number: String },
    ClickedSendCode,
    ClickedVerifyCode { phone_number: String },
}
