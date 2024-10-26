use std::env;
use std::fs;
use std::io::Error;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

const DBMATE_VERSION: &str = "1.14.0";

fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn download_dbmate() -> Result<String, Box<dyn std::error::Error>> {
    let filename = format!("dbmate-{}", DBMATE_VERSION);
    let url = format!(
        "https://github.com/amacneil/dbmate/releases/download/v{}/dbmate-macos-amd64",
        DBMATE_VERSION
    );
    let filepath = format!("{}/{}", env::current_dir()?.display(), filename);

    println!("Downloading dbmate version {}...", DBMATE_VERSION);

    if command_exists("curl") {
        let curl_status = Command::new("curl")
            .arg("-L")
            .arg("-o")
            .arg(&filepath)
            .arg(&url)
            .status()?;

        if !curl_status.success() {
            return Err(Box::new(Error::new(
                std::io::ErrorKind::Other,
                "Failed to download dbmate using curl",
            )));
        }
    } else if command_exists("wget") {
        let wget_status = Command::new("wget")
            .arg("-O")
            .arg(&filepath)
            .arg(&url)
            .status()?;

        if !wget_status.success() {
            return Err(Box::new(Error::new(
                std::io::ErrorKind::Other,
                "Failed to download dbmate using wget",
            )));
        }
    } else {
        return Err(Box::new(Error::new(
            std::io::ErrorKind::NotFound,
            "Neither curl nor wget is installed",
        )));
    }

    let mut perms = fs::metadata(&filepath)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&filepath, perms)?;

    Ok(filepath)
}

fn run_dbmate_up(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let version_output = Command::new(filepath).arg("--version").output()?;
    println!("{}", String::from_utf8_lossy(&version_output.stdout));

    let up_output = Command::new(filepath).arg("up").output()?;
    println!("{}", String::from_utf8_lossy(&up_output.stdout));

    fs::remove_file(filepath)?;

    Ok(())
}

pub fn up() -> Result<(), Box<dyn std::error::Error>> {
    let filepath = download_dbmate()?;
    run_dbmate_up(&filepath)
}
