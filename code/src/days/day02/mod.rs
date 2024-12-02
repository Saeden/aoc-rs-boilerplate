use crate::PuzzlePart;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn run(part: PuzzlePart, input: &Path) {
    match part {
        PuzzlePart::A => day2_1(input),
        PuzzlePart::B => day2_2(input),
    }
}

fn day2_2(input: &Path) {
    let file = File::open(input).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: usize = 0;

    for line in reader.lines() {
        let mut balls: Balls = Balls {
            red: 0,
            green: 0,
            blue: 0,
        };
        get_min_balls(&line, &mut balls);

        let power = balls.red * balls.green * balls.blue;
        output += power;
        // println!("MinBalls: {:?}\nPower: {}\nOutput: {}", balls, power, output);
    }
    println!("\nOutput 2: {}", output)
}

fn get_min_balls(line_result: &Result<String, Error>, balls: &mut Balls) {
    let line = line_result.as_ref().expect("Error getting line");
    let tokens: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .collect();
    for i in (4..tokens.len()).step_by(3) {
        let colour = tokens[i];
        if let Ok(num) = tokens[i - 1].parse::<usize>() {
            // println!("{:?} {}", balls, colour);
            if colour == "red" && num > balls.red {
                balls.red = num;
            } else if colour == "green" && num > balls.green {
                balls.green = num;
            } else if colour == "blue" && num > balls.blue {
                balls.blue = num;
            } else if !["red", "green", "blue"].contains(&colour) {
                println!(
                    "Bad tokens. Balls: {} | Colour: {}",
                    tokens[i - 1],
                    tokens[i]
                )
            }
        } else {
            panic!(
                "Bad tokens. Balls: {} | Colour: {}",
                tokens[i - 1],
                tokens[i]
            )
        }
    }
}

#[derive(Debug)]
struct Balls {
    red: usize,
    green: usize,
    blue: usize,
}

fn day2_1(input: &Path) {
    let file = File::open(input).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: usize = 0;

    for line in reader.lines() {
        let complete = check_game(&line);

        if complete {
            let id = get_id(&line);
            // println!("id: {}", id);
            output += id as usize;
        }
    }
    println!("\nOutput 1: {}", output)
}

fn check_game(line_result: &Result<String, Error>) -> bool {
    let line = line_result.as_ref().expect("Error getting line");
    let tokens: Vec<&str> = line
        .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
        .collect();
    for i in (4..tokens.len()).step_by(3) {
        let colour = tokens[i];
        if let Ok(balls) = tokens[i - 1].parse::<u32>() {
            // println!("{:?} {}", balls, colour);
            if colour == "red" && balls > 12 {
                return false;
            } else if colour == "green" && balls > 13 {
                return false;
            } else if colour == "blue" && balls > 14 {
                return false;
            } else if !["red", "green", "blue"].contains(&colour) {
                println!(
                    "Bad tokens. Balls: {} | Colour: {}",
                    tokens[i - 1],
                    tokens[i]
                )
            }
        } else {
            panic!(
                "Bad tokens. Balls: {} | Colour: {}",
                tokens[i - 1],
                tokens[i]
            )
        }
    }
    return true;
}

fn get_id(line_result: &Result<String, Error>) -> u8 {
    let mut digits: Vec<u8> = Vec::new();
    let mut found = false;
    let mut id = 0;

    let line = line_result.as_ref().expect("Error getting line");
    for char in line.chars() {
        if found == true && char != ':' {
            digits.push((char as u8) - ('0' as u8))
        } else if char == ' ' {
            found = true;
        } else if char == ':' {
            break;
        }
    }

    for i in 0..=digits.len() - 1 {
        let power = digits.len() - i - 1;
        id += digits[i] * 10_u8.pow(power as u32);
        // println!("Digit: {}, Power: {}, ID: {}", digits[i], power, id);
    }
    return id;
}
