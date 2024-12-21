use std::{
    fs::{self, OpenOptions},
    io::{self, Write}
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{get_base_directory, get_log_file_path};

#[derive(Debug, Serialize, Deserialize)]
struct CommandEntry {
    command: String,
    timestamp: DateTime<Utc>
}

pub fn log_command(command: String) -> io::Result<()> {
    if !should_log_command(&command) {
        return  Ok(());
    }

    let entry = CommandEntry {
        command,
        timestamp: Utc::now(),
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_log_file_path()?)?;

    writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    Ok(())
}

pub fn clear_logs() -> io::Result<()> {
    print!("Clear all command logs across all years? This action is irreversible [y/n] > ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let base_path = get_base_directory()?;
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    println!("Command logs cleared");
    Ok(())
}

fn should_log_command(command: &str) -> bool {
    !command.trim().starts_with("cli-wrapped") &&
    !command.trim().is_empty() &&
    !command.trim().starts_with("code")
}