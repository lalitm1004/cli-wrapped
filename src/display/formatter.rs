use std::{
    fs,
    collections::HashMap,
    io::{self, BufRead},
    path::Path,
};
use colored::{Color, Colorize};
use walkdir::WalkDir;

use crate::{
    LOG_FILE_EXTENSION,
    LOG_FILE_PREFIX,
    get_log_directory,
    CommandEntry,
};

use super::ascii::{print_title, print_year};

pub fn display_wrapped() -> io::Result<()> {
    let (command_map, invocation_map) = get_command_invocation_maps()?;
    let total_commands: usize = command_map.values().sum();
    let sorted_commands = sort_map(command_map);
    let sorted_invocations = sort_map(invocation_map);

    println!("\n");
    print_title();
    print_year();

    println!("\n");
    println!("     Count | Top Commands                  Count | Top Invocations");
    println!("   --------+------------------------     --------+--------------------------------");
    print_top_10(sorted_commands, sorted_invocations);

    println!("\n   Total Commands > {}", total_commands.to_string().green());

    Ok(())
}

fn print_top_10(
    sorted_commands: Vec<(String, usize)>,
    sorted_invocations: Vec<(String, usize)>
) {

    let max_entries = 10;
    let commands_len = sorted_commands.len();
    let invocations_len = sorted_invocations.len();

    let num_rows = max_entries.min(commands_len.max(invocations_len));

    for i in 0..num_rows {
        // let formatted_command = format_entry(sorted_commands[i].clone());
        let formatted_command = if i < commands_len {
            format_entry(sorted_commands[i].clone())
        } else {
            String::from(" ".repeat(5 + 24 + 4))
        };

        let formatted_invocation: String = if i < invocations_len {
            format_entry(sorted_invocations[i].clone())
        } else {
            String::from(" ".repeat(5 + 24))
        };

        println!(
            "    {}     {}",
            formatted_command,
            formatted_invocation,
        )
    }
}

fn format_entry(entry: (String, usize)) -> String {
    let gray: Color = color_from_hex("#525252");
    let text: Color = color_from_hex("#6D28D9");

    let count = format!("{:06}", entry.1);
    let leading_zeroes = &count[0..count.find(|c: char| c != '0').unwrap_or(count.len())];
    let rest = &count[leading_zeroes.len()..];

    format!(
        "{}{} | {:24}",
        leading_zeroes.color(gray),
        rest,
        entry.0.color(text)
    )
}

fn get_command_invocation_maps() -> io::Result<(HashMap<String, usize>, HashMap<String, usize>)> {

    let mut command_map: HashMap<String, usize> = HashMap::new();
    let mut invocation_map: HashMap<String, usize> = HashMap::new();

    let log_directory = get_log_directory()?;
    for entry in WalkDir::new(log_directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_name = entry.file_name().to_string_lossy();
        if file_name.starts_with(LOG_FILE_PREFIX) && file_name.ends_with(LOG_FILE_EXTENSION) {
            tally_log_file(entry.path(), &mut command_map, &mut invocation_map);
        }
    }

    Ok((command_map, invocation_map))
}

fn tally_log_file(
    log_file_path: &Path,
    command_map: &mut HashMap<String, usize>,
    invocation_map: &mut HashMap<String, usize>
) {
    if let Ok(file) = fs::File::open(log_file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines().filter_map(|l| l.ok()) {
            if let Ok(entry) = serde_json::from_str::<CommandEntry>(&line) {
                let mut split_command = entry.command.split_whitespace();
                if let Some(base_command) = split_command.next() {
                    // custom parsing for python to handle virtual environments
                    if base_command.ends_with("python.exe") {
                        *command_map.entry("python".to_string()).or_insert(0) += 1;
                        *invocation_map.entry(format!("python {:?}", split_command.next().unwrap()).to_string()).or_insert(0) += 1;
                    } else {
                        *command_map.entry(base_command.to_string()).or_insert(0) += 1;
                        *invocation_map.entry(entry.command).or_insert(0) += 1;
                    }
                }
            }
        }
    }
}

fn sort_map(map: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_map: Vec<(String, usize)> = map.into_iter().collect();
    sorted_map.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_map
}

fn color_from_hex(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        panic!("ERROR: invalid hex code: {}", hex);
    }

    let r = u8::from_str_radix(&hex[0..2], 16).expect("ERROR: invalid hex code");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("ERROR: invalid hex code");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("ERROR: invalid hex code");

    Color::TrueColor { r, g, b }
}