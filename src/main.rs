use clap::Parser;
use days::PuzzlePart;
use std::path::{self, Path};
use std::process::exit;
use utils::line_iterator;

// Placeholder for dynamic function loading
mod days;
mod utils;

#[derive(Parser, Debug)]
struct Args {
    day: u8,

    part: PuzzlePart,

    input: String,
}

fn main() {
    // Setup CLI
    let args = Args::parse();

    let day = args.day;
    let part = args.part;
    let input_str = args.input;

    // Construct file paths
    let file_str = format_path(day, input_str);
    let file_path = path::absolute(&file_str).expect("Cannot create absolute path");

    if !file_path.exists() {
        eprintln!("Error: Could not find {}", file_path.display());
        exit(1);
    }

    let input = line_iterator(file_path.as_path());
    // Run the corresponding function
    days::run_day(day, part, input);
}

fn format_path(day: u8, input: String) -> String {
    if day < 10 && day > 0 {
        format!("day0{}/{}.txt", day, input)
    } else if day <= 25 && day > 0 {
        format!("day{}/{}.txt", day, input)
    } else {
        eprintln!("Error: <DAY> needs to be between 1 and 25",);
        exit(1);
    }
}
