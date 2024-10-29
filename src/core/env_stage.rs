#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvSage {
    Dev,
    Prod,
}

impl EnvSage {
    pub fn from_str(s: &str) -> Self {
        let cleaned = s.to_ascii_lowercase();

        if cleaned.contains("dev") {
            return EnvSage::Dev;
        }

        if cleaned.contains("prod") {
            return EnvSage::Prod;
        }

        EnvSage::Dev
    }

    pub fn is_prod(&self) -> bool {
        *self == EnvSage::Prod
    }

    pub fn is_dev(&self) -> bool {
        *self == EnvSage::Dev
    }
}
