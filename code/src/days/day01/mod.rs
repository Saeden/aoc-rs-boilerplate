use crate::PuzzlePart;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use utils::line_iterator;

pub fn run(part: PuzzlePart, input: &Path) {
    match part {
        PuzzlePart::A => part_a(input),
        PuzzlePart::B => part_b(input),
    }
}

fn part_b(input: &Path) {
    let mut output: Vec<usize> = Vec::new();

    for line in line_iterator(input) {
        let line = line.expect("Failed to read line");
        let mut digits: Vec<usize> = Vec::new();
        let mut sub_string: String = String::new();

        for character in line.chars() {
            if character.is_ascii_digit() {
                digits.push((character as usize) - ('0' as usize));
            } else if character.is_alphabetic() {
                sub_string = sub_string + &String::from(character);
                if check_str(&mut sub_string) {
                    let found_digit = convert_to_digit(&mut sub_string);
                    digits.push(found_digit);
                }
            }
        }
        let mut digit = digits[0] * 10;
        digit += digits[digits.len() - 1];
        output.push(digit);
    }

    let num: usize = output.iter().sum();
    println!("{}", num);
}

fn check_str(sub_string: &mut String) -> bool {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for number in numbers {
        if sub_string.len() > number.len() {
            continue;
        } else if *sub_string == number {
            return true;
        } else if number.to_string()[..sub_string.len()] == *sub_string {
            return false;
        }
    }
    *sub_string = sub_string[1..].to_string();
    false
}

fn convert_to_digit(sub_string: &mut String) -> usize {
    match sub_string.as_str() {
        "one" => {
            *sub_string = String::from("e");
            1
        }
        "two" => {
            *sub_string = String::from("o");
            2
        }
        "three" => {
            *sub_string = String::from("e");
            3
        }
        "four" => {
            *sub_string = String::from("");
            4
        }
        "five" => {
            *sub_string = String::from("e");
            5
        }
        "six" => {
            *sub_string = String::from("");
            6
        }
        "seven" => {
            *sub_string = String::from("n");
            7
        }
        "eight" => {
            *sub_string = String::from("t");
            8
        }
        "nine" => {
            *sub_string = String::from("e");
            9
        }
        _ => panic!("Something is triggering as a digit: {}", sub_string),
    }
}

fn part_a(input: &Path) {
    let file = File::open(input).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut digits: Vec<usize> = Vec::new();
        for character in line.chars() {
            if character.is_ascii_digit() {
                digits.push((character as usize) - ('0' as usize));
            }
        }
        let mut digit = digits[0] * 10;
        digit += digits[digits.len() - 1];
        output.push(digit);
    }

    let num: usize = output.iter().sum();
    println!("{}", num);
}
