use async_trait::async_trait;

use super::interface::VerifySms;

pub struct ImplFake {
    code: String,
}

impl ImplFake {
    pub fn new() -> Self {
        ImplFake {
            code: "123456".to_string(),
        }
    }

    pub fn code(mut self, value: &str) -> Self {
        self.code = value.to_string();
        self
    }
}

#[async_trait]
impl VerifySms for ImplFake {
    async fn send_code(&self, _phone_number: &str) -> Result<(), std::io::Error> {
        Ok(())
    }

    async fn verify_code(&self, _phone_number: &str, _code: &str) -> Result<(), std::io::Error> {
        Ok(())
    }
}
