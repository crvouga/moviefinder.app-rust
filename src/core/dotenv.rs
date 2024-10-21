use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

        // Skip empty lines or lines starting with '#'
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Split the line into key and value
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();

        // Set the environment variable
        env::set_var(key, value);
    }

    Ok(())
}
