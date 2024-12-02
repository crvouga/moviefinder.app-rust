use async_trait::async_trait;

pub enum VerifyCodeError {
    WrongCode,
    Error(std::io::Error),
}

impl VerifyCodeError {
    pub fn error(self) -> std::io::Error {
        match self {
            VerifyCodeError::WrongCode => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "Wrong code")
            }
            VerifyCodeError::Error(err) => err,
        }
    }
}

#[async_trait]
pub trait VerifySms: Send + Sync {
    async fn send_code(&self, phone_number: &str) -> Result<(), std::io::Error>;
    async fn verify_code(&self, phone_number: &str, code: &str) -> Result<(), VerifyCodeError>;
}
