use crate::core;

#[derive(PartialEq, Eq)]
pub enum TestEnv {
    Unit,
    Integration,
    None,
}

impl TestEnv {
    pub fn from_str(s: &str) -> TestEnv {
        match s.to_ascii_lowercase().as_str() {
            "unit" => TestEnv::Unit,
            "integration" => TestEnv::Integration,
            _ => TestEnv::None,
        }
    }
}

pub struct Env {
    pub tmdb_api_read_access_token: String,
    pub port: String,
    pub test_env: TestEnv,
}

impl Env {
    pub fn load() -> Env {
        core::env::load().unwrap_or_default();

        let tmdb_api_read_access_token = core::env::read("TMDB_API_READ_ACCESS_TOKEN");
        let port = core::env::read("PORT");
        let test_env = TestEnv::from_str(&core::env::read_or_default("TEST_ENV"));

        Env {
            tmdb_api_read_access_token,
            port,
            test_env,
        }
    }
}
