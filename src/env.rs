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
    pub simulate_latency: bool,

    #[allow(dead_code)]
    pub test_env: TestEnv,
}

impl Env {
    pub fn load() -> Option<Env> {
        let env = core::env::load(".env").unwrap();
        println!("LOG env {:?}", env);
        let env_stage = EnvSage::from_str(env.get("STAGE").unwrap_or(&"".to_string()));
        let tmdb_api_read_access_token = env
            .get("TMDB_API_READ_ACCESS_TOKEN")
            .cloned()
            .unwrap_or("".to_string());
        let port = env.get("PORT").cloned().unwrap_or("".to_string());
        let test_env = TestEnv::from_str(&env.get("TEST_ENV").cloned().unwrap_or("".to_string()));
        let database_url = env.get("DATABASE_URL").cloned()?;

        let simulate_latency = env_stage.is_dev();

        if simulate_latency {
            println!("LOG Simulating latency");
        }

        Some(Env {
            simulate_latency,
            database_url,
            tmdb_api_read_access_token,
            port,
            test_env,
        })
    }
}
