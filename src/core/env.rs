use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} file not found", file_path),
        ));
    }

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap_or("").trim().to_string();
        let value = remove_quotes(parts.next().unwrap_or("").trim()).to_string();

        env::set_var(key, value);
    }

    Ok(())
}

fn remove_quotes(s: &str) -> &str {
    if s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
