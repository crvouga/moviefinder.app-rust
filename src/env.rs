use crate::core;

#[derive(PartialEq, Eq, Clone)]
pub enum TestEnv {
    Unit,
    Integration,
    None,
}

impl TestEnv {
    pub fn from_str(s: &str) -> TestEnv {
        let cleaned = s.to_ascii_lowercase();

        if cleaned.contains("unit") {
            return TestEnv::Unit;
        }

        if cleaned.contains("int") {
            return TestEnv::Integration;
        }

        TestEnv::None
    }
}

pub struct Env {
    pub tmdb_api_read_access_token: String,
    pub port: String,
    pub database_url: String,
    pub simulate_latency: bool,
    pub is_prod: bool,

    #[allow(dead_code)]
    pub test_env: TestEnv,
}

impl Env {
    pub fn load() -> Option<Env> {
        core::env::load().unwrap_or_default();

        let tmdb_api_read_access_token = core::env::read("TMDB_API_READ_ACCESS_TOKEN")?;
        let port = core::env::read("PORT")?;
        let test_env = TestEnv::from_str(&core::env::read("TEST_ENV").unwrap_or("".to_string()));
        let database_url = core::env::read("DATABASE_URL")?;
        let is_prod = core::env::read("RUST_ENV").unwrap_or("".to_string()) == "production";

        let simulate_latency = !is_prod;

        Some(Env {
            is_prod,
            simulate_latency,
            database_url,
            tmdb_api_read_access_token,
            port,
            test_env,
        })
    }
}
