use async_trait::async_trait;

pub enum SendCodeError {
    InvalidPhoneNumber,
    Error(crate::core::error::Error),
}

pub enum VerifyCodeError {
    WrongCode,
    Error(crate::core::error::Error),
}

#[async_trait]
pub trait VerifySms: Send + Sync {
    async fn send_code(&self, phone_number: &str) -> Result<(), SendCodeError>;
    async fn verify_code(&self, phone_number: &str, code: &str) -> Result<(), VerifyCodeError>;
}
