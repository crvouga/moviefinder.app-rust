use async_trait::async_trait;

use crate::core::random;

use super::interface::{VerifyCodeError, VerifySms};

pub struct ImplFake {
    correct_code: String,
}

impl ImplFake {
    pub fn new() -> Self {
        ImplFake {
            correct_code: "123".to_string(),
        }
    }

    pub fn correct_code(mut self, value: &str) -> Self {
        self.correct_code = value.to_string();
        self
    }
}

#[async_trait]
impl VerifySms for ImplFake {
    async fn send_code(&self, _phone_number: &str) -> Result<(), std::io::Error> {
        let should_error = false;
        if should_error {
            let err = std::io::Error::new(std::io::ErrorKind::Other, "Some fake error");
            return Err(err);
        }
        Ok(())
    }

    async fn verify_code(&self, _phone_number: &str, code: &str) -> Result<(), VerifyCodeError> {
        let should_error = false;
        if should_error {
            let err = std::io::Error::new(std::io::ErrorKind::Other, "Some fake error");
            return Err(VerifyCodeError::Error(err));
        }
        if self.correct_code != code {
            return Err(VerifyCodeError::WrongCode);
        }
        Ok(())
    }
}
