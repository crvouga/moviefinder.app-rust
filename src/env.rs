use crate::core::{self, env_stage::EnvStage};
use std::{env, time::Duration};

pub struct Env {
    pub tmdb_api_read_access_token: String,
    pub port: String,
    pub database_url: String,
    pub simulate_latency: Option<Duration>,
    #[cfg(test)]
    pub test_env: TestEnv,
}

impl Env {
    pub fn load() -> Env {
        core::env::load(".env").unwrap();

        let env_stage = EnvStage::from_str(env::var("STAGE").unwrap_or("".to_string()).as_str());

        if env_stage.is_dev() {
            core::env::load(".env.local").unwrap_or(());
        }

        let tmdb_api_read_access_token = env::var("TMDB_API_READ_ACCESS_TOKEN").unwrap();

        let port = env::var("PORT").unwrap();

        let database_url = env::var("DATABASE_URL").unwrap();

        let simulate_latency_duration = Duration::from_millis(100);

        let simulate_latency = if env_stage.is_dev() {
            Some(simulate_latency_duration)
        } else {
            None
        };

        #[cfg(test)]
        let test_env = TestEnv::from_str(&env::var("TEST_ENV").unwrap_or("".to_string()));

        let env = Env {
            simulate_latency,
            database_url,
            tmdb_api_read_access_token,
            port,
            #[cfg(test)]
            test_env,
        };

        env
    }
}

#[cfg(test)]
#[derive(PartialEq, Eq, Clone)]
pub enum TestEnv {
    Unit,
    Integration,
    None,
}

#[cfg(test)]
impl TestEnv {
    pub fn is_integration(&self) -> bool {
        self == &TestEnv::Integration
    }
}

#[cfg(test)]
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
