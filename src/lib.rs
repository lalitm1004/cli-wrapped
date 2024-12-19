use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, Write},
    path::PathBuf
};

use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

const MAX_LINES: usize = 100;

#[derive(Debug, Serialize, Deserialize)]
struct CommandEntry {
    command: String,
    timestamp: DateTime<Utc>,
}


pub fn log_command(command: String) -> io::Result<()> {
    if !should_log_command(&command) {
        return Ok(());
    }

    let entry = CommandEntry {
        command,
        timestamp: Utc::now(),
    };

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(get_log_file_path())?;

    writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    Ok(())
}

pub fn clear_log() -> io::Result<()> {
    print!("This action is irreversible. Clear command logs? [y/n] > ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;


    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Operation cancelled");
        return Ok(());
    }

    let mut base_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base_path.push("wrapped");

    if base_path.exists() {
        for entry in fs::read_dir(&base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }

    println!("Command logs cleared");

    Ok(())
}

pub fn display_wrapped() -> io::Result<()> {
    println!("display");

    Ok(())
}

fn get_log_directory() -> PathBuf {
    let mut base_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base_path.push("wrapped");

    // make current year subfolder
    let year = Utc::now().year().to_string();
    base_path.push(year);
    std::fs::create_dir_all(&base_path).unwrap();

    base_path
}

fn get_log_file_path() -> PathBuf {
    let log_dir = get_log_directory();

    let mut highest_num = 0;
    for entry in WalkDir::new(&log_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.starts_with("command_log_") && file_name.ends_with(".jsonl") {
            if let Some(num_str) = file_name
                .strip_prefix("command_log_")
                .and_then(|s| s.strip_suffix(".jsonl"))
            {
                if let Ok(num) = num_str.parse::<usize>() {
                    highest_num = highest_num.max(num);
                }
            }
        }
    }

    let mut latest_file = log_dir.clone();
    latest_file.push(format!("command_log_{}.jsonl", highest_num));

    if !latest_file.exists() || count_lines(&latest_file) < MAX_LINES {
        latest_file
    } else {
        let mut new_file = log_dir;
        new_file.push(format!("command_log_{}.jsonl", highest_num + 1));
        new_file
    }
}


fn count_lines(file_path: &PathBuf) -> usize {
    if !file_path.exists() {
        return 0;
    }

    let file = fs::File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines().count()
}

fn should_log_command(command: &str) -> bool {
    !command.trim().starts_with("wrapped") &&
    !command.trim().is_empty()
}