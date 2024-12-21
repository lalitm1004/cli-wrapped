use std::{fs, io::{self, BufRead}, path::PathBuf};

use chrono::{Datelike, Utc};
use walkdir::WalkDir;

pub const LOG_FOLDERS_DIR: &str = "cli-wrapped";
pub const LOG_FILE_PREFIX: &str = "command_log_";
pub const LOG_FILE_EXTENSION: &str = ".jsonl";
pub const MAX_LINES: usize = 100;

pub fn get_base_directory() -> io::Result<PathBuf> {
    let mut base_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base_path.push(LOG_FOLDERS_DIR);
    fs::create_dir_all(&base_path)?;
    Ok(base_path)
}

pub fn get_log_directory() -> io::Result<PathBuf> {
    let mut base_path = get_base_directory()?;
    let year = Utc::now().year().to_string();
    base_path.push(year);
    fs::create_dir_all(&base_path)?;
    Ok(base_path)
}

pub fn get_log_file_path() -> io::Result<PathBuf> {
    let log_dir = get_log_directory()?;
    let highest_num = get_highest_log_number(&log_dir)?;

    let mut latest_file = log_dir.clone();
    latest_file.push(format!("{}{}{}", LOG_FILE_PREFIX, highest_num, LOG_FILE_EXTENSION));

    if !latest_file.exists() || count_lines(&latest_file)? < MAX_LINES {
        Ok(latest_file)
    } else {
        let mut new_file = log_dir;
        new_file.push(format!("{}{}{}", LOG_FILE_PREFIX, highest_num + 1, LOG_FILE_EXTENSION));
        Ok(new_file)
    }
    }

fn get_highest_log_number(log_dir: &PathBuf) -> io::Result<usize> {
    let mut highest_num = 0;

    for entry in WalkDir::new(log_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if let Some(num) = extract_log_number(&file_name) {
            highest_num = highest_num.max(num);
        }
    }

    Ok(highest_num)
}

fn extract_log_number(filename: &str) -> Option<usize> {
    if !filename.starts_with(LOG_FILE_PREFIX) || !filename.ends_with(LOG_FILE_EXTENSION) {
        return None;
    }

    filename
        .strip_prefix(LOG_FILE_PREFIX)?
        .strip_suffix(LOG_FILE_EXTENSION)?
        .parse::<usize>()
        .ok()
}

fn count_lines(file_path: &PathBuf) -> io::Result<usize> {
    if !file_path.exists() {
        return  Ok(0);
    }

    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}