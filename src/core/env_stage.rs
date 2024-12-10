#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvStage {
    Local,
    Prod,
}

impl EnvStage {
    pub fn from_str(s: &str) -> Self {
        let cleaned = s.to_ascii_lowercase();

        if cleaned.contains("local") {
            return EnvStage::Local;
        }

        if cleaned.contains("prod") {
            return EnvStage::Prod;
        }

        EnvStage::Prod
    }

    pub fn is_local(&self) -> bool {
        *self == EnvStage::Local
    }
}
