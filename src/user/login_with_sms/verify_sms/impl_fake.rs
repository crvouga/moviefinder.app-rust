use async_trait::async_trait;

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

    // pub fn correct_code(mut self, value: &str) -> Self {
    //     self.correct_code = value.to_string();
    //     self
    // }
}

const SHOULD_SLEEP: bool = true;
const SHOULD_ERROR_SEND_CODE: bool = false;
const SHOULD_ERROR_VERIFY_CODE: bool = false;

#[async_trait]
impl VerifySms for ImplFake {
    async fn send_code(&self, _phone_number: &str) -> Result<(), std::io::Error> {
        if SHOULD_SLEEP {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        if SHOULD_ERROR_SEND_CODE {
            let err = std::io::Error::new(std::io::ErrorKind::Other, "Sending code failed");
            return Err(err);
        }

        Ok(())
    }

    async fn verify_code(&self, _phone_number: &str, code: &str) -> Result<(), VerifyCodeError> {
        if SHOULD_SLEEP {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }

        if SHOULD_ERROR_VERIFY_CODE {
            let err = std::io::Error::new(std::io::ErrorKind::Other, "Verifying code failed");
            return Err(VerifyCodeError::Error(err));
        }

        if self.correct_code != code {
            return Err(VerifyCodeError::WrongCode);
        }

        Ok(())
    }
}
