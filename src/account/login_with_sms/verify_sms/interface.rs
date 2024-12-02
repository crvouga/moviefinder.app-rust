use async_trait::async_trait;

#[async_trait]
pub trait VerifySms {
    async fn send_code(&self, phone_number: &str) -> Result<(), std::io::Error>;
    async fn verify_code(&self, phone_number: &str, code: &str) -> Result<(), std::io::Error>;
}
