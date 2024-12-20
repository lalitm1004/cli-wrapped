use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{self, BufRead, Write},
    path::{Path, PathBuf}
};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use colored::*;

pub mod ascii;

const MAX_LINES: usize = 100;
const GRAY_COLOR: Color = Color::TrueColor { r: 128, g: 128, b: 128 };
const COLOR_2: Color = Color::TrueColor { r: 142, g: 57, b: 189 };

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
    print!("This action is irreversible. Clear command logs across all years? [y/n] > ");
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
    let log_dir = get_log_directory();
    let mut command_map: HashMap<String, usize> = HashMap::new();
    let mut invokation_map: HashMap<String, usize> = HashMap::new();

    for entry in WalkDir::new(&log_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.starts_with("command_log_") && file_name.ends_with(".jsonl") {
            tally_log_file(
                entry.path(),
                &mut command_map,
                &mut invokation_map,
            );
        }
    }
    
    let total_commands: usize = invokation_map.values().sum();

    let sorted_command = sort_map(command_map);
    let sorted_invokation = sort_map(invokation_map);

    ascii::print_title();
    ascii::print_year();


    println!("\nTop Commands");
    println!("-----------------");
    display_top_10(sorted_command);
    
    println!("\nTop Invokations");
    println!("-----------------");
    display_top_10(sorted_invokation);

    println!(
        "\nTotal Commands > {}",
        total_commands.to_string().green()
    );
    // println!("{:?}", sorted_command);
    // println!("{:?}", sorted_invokation);

    Ok(())
}

fn display_top_10(sorted: Vec<(String, usize)>) {
    for i in 0..10.min(sorted.len()) {
        let count = format!("{:06}", sorted[i].1);
        let leading_zeroes = &count[0..count.find(|c: char| c != '0').unwrap_or(count.len())];
        let rest = &count[leading_zeroes.len()..];

        println!(
            "{}{} {}",
            leading_zeroes.color(GRAY_COLOR),
            rest,
            sorted[i].0.color(COLOR_2)
        )
    }
}

fn sort_map(map: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_map: Vec<(String, usize)> = map.into_iter().collect();
    sorted_map.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_map
}

fn tally_log_file(
    log_file_path: &Path,
    command_map: &mut HashMap<String, usize>,
    invokation_map: &mut HashMap<String, usize>
) {
    if let Ok(file) = fs::File::open(log_file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines().filter_map(|l| l.ok()) {
            if let Ok(entry) = serde_json::from_str::<CommandEntry>(&line) {
                if let Some(base_command) = entry.command.split_whitespace().next() {
                    *command_map.entry(base_command.to_string()).or_insert(0) += 1;
                    *invokation_map.entry(entry.command).or_insert(0) += 1;
                }
            }
        }
    }
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

    // find number of latest log file
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
    !command.trim().starts_with("cli-wrapped") &&
    !command.trim().is_empty() &&
    !command.trim().starts_with("code")
}