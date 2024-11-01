use std::{env, time::Duration};

use crate::core::{self, env_stage::EnvSage};

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
    pub simulate_latency: Option<Duration>,

    #[allow(dead_code)]
    pub test_env: TestEnv,
}

impl Env {
    pub fn load() -> Option<Env> {
        core::env::load(".env").unwrap_or(());
        let env_stage = EnvSage::from_str(env::var("STAGE").unwrap_or("".to_string()).as_str());

        if env_stage == EnvSage::Dev {
            core::env::load(".env.local").unwrap_or(());
        }

        let tmdb_api_read_access_token =
            env::var("TMDB_API_READ_ACCESS_TOKEN").unwrap_or("".to_string());
        let port = env::var("PORT").unwrap_or("".to_string());
        let test_env = TestEnv::from_str(&env::var("TEST_ENV").unwrap_or("".to_string()));
        let database_url = env::var("DATABASE_URL").unwrap_or("".to_string());

        let simulate_latency_duration = Duration::from_millis(100);

        let simulate_latency = if env_stage.is_dev() {
            Some(simulate_latency_duration)
        } else {
            None
        };

        let env = Env {
            simulate_latency,
            database_url,
            tmdb_api_read_access_token,
            port,
            test_env,
        };

        Some(env)
    }
}
