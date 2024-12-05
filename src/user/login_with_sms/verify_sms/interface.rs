use async_trait::async_trait;

pub enum VerifyCodeError {
    WrongCode,
    Error(std::io::Error),
}

#[async_trait]
pub trait VerifySms: Send + Sync {
    async fn send_code(&self, phone_number: &str) -> Result<(), std::io::Error>;
    async fn verify_code(&self, phone_number: &str, code: &str) -> Result<(), VerifyCodeError>;
}
