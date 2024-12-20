use crate::core::{self, env_stage::EnvStage};
use std::{env, time::Duration};

pub struct Env {
    pub tmdb_api_read_access_token: String,
    pub port: String,
    pub database_url: String,
    pub simulate_latency: Option<Duration>,
    pub twilio_service_sid: String,
    pub twilio_auth_token: String,
    pub twilio_account_sid: String,
    pub stage: EnvStage,
    pub test_env: TestEnv,
}

impl Env {
    pub fn load() -> Env {
        let stage = EnvStage::from_str(env::var("STAGE").unwrap_or("".to_string()).as_str());

        core::env::load(".env").unwrap_or(());

        let tmdb_api_read_access_token = env::var("TMDB_API_READ_ACCESS_TOKEN").unwrap();

        let port = env::var("PORT").unwrap_or("3000".to_owned());

        let database_url = env::var("DATABASE_URL").unwrap();

        if database_url.is_empty() {
            panic!("DATABASE_URL must be set");
        }

        let simulate_latency_duration = Duration::from_millis(75);

        let simulate_latency = if stage.is_local() {
            Some(simulate_latency_duration)
        } else {
            None
        };

        let test_env = TestEnv::from_str(&env::var("TEST_ENV").unwrap_or("".to_string()));

        let twilio_account_sid = env::var("TWILIO_ACCOUNT_SID").unwrap();

        let twilio_auth_token = env::var("TWILIO_AUTH_TOKEN").unwrap();

        let twilio_service_sid = env::var("TWILIO_SERVICE_SID").unwrap();

        let env = Env {
            simulate_latency,
            database_url,
            tmdb_api_read_access_token,
            port,
            twilio_account_sid,
            twilio_auth_token,
            twilio_service_sid,
            stage,
            test_env,
        };

        env
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum TestEnv {
    Unit,
    Integration,
    None,
}

impl TestEnv {
    #[allow(dead_code)]
    pub fn is_integration(&self) -> bool {
        self == &TestEnv::Integration
    }

    pub fn is_unit(&self) -> bool {
        self == &TestEnv::Unit
    }
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
