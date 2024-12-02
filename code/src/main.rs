use clap::Parser;
use days::PuzzlePart;
use std::path::Path;
use std::process::exit;

// Placeholder for dynamic function loading
mod days;

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
    let file_path = Path::new(&file_str);

    if !file_path.exists() {
        eprintln!("Error: Could not find {}", file_path.display());
        exit(1);
    }

    // Run the corresponding function
    days::run_day(day, part, file_path);
}

fn format_path(day: u8, input: String) -> String {
    if day < 10 && day > 0 {
        format!(
            "/home/matt/repos/aoc-rs-2023/code/src/days/day0{}/{}.txt",
            day, input
        )
    } else if day <= 25 && day > 0 {
        format!(
            "/home/matt/repos/aoc-rs-2023/code/src/days/day{}/{}.txt",
            day, input
        )
    } else {
        eprintln!("Error: <DAY> needs to be between 1 and 25",);
        exit(1);
    }
}
