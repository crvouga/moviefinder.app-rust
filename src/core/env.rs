use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn remove_quotes(s: &str) -> &str {
    if s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

pub fn load() -> io::Result<()> {
    let env_path = ".env";

    if !Path::new(env_path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            ".env file not found",
        ));
    }

    let file = File::open(env_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();
        env::set_var(key, remove_quotes(value));
    }

    Ok(())
}

pub fn read(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}

pub fn read_or_default(key: &str) -> String {
    env::var(key).unwrap_or_default()
}
