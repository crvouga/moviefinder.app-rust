#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvStage {
    Dev,
    Prod,
}

impl EnvStage {
    pub fn from_str(s: &str) -> Self {
        let cleaned = s.to_ascii_lowercase();

        if cleaned.contains("dev") {
            return EnvStage::Dev;
        }

        if cleaned.contains("prod") {
            return EnvStage::Prod;
        }

        EnvStage::Dev
    }

    pub fn is_dev(&self) -> bool {
        *self == EnvStage::Dev
    }
}
