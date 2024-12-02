use crate::PuzzlePart;
use std::path::Path;
use utils::line_iterator;

use std::{char, collections::HashMap};

pub fn run(part: PuzzlePart, input: &Path) {
    match part {
        PuzzlePart::A => part_a(input),
        PuzzlePart::B => part_b(input),
    }
}

fn _pretty_print_gear(gear: &Point, map: &HashMap<Point, char>) {
    let mut first_line_buf: String = String::new();
    let mut second_line_buf: String = String::new();
    let mut third_line_buf: String = String::new();

    for (i, neighbour) in all_neighbour_points(gear.x, gear.y).iter().enumerate() {
        let nb_result = map.get(neighbour);
        if let Some(result) = nb_result {
            match i {
                0 => {
                    first_line_buf.push(*result);
                }
                1 => {
                    second_line_buf.push(*result);
                    second_line_buf.push('*');
                }
                2 => third_line_buf.push(*result),
                3 => first_line_buf.push(*result),
                4 => third_line_buf.push(*result),
                5 => first_line_buf.push(*result),
                6 => second_line_buf.push(*result),
                7 => third_line_buf.push(*result),
                _ => (),
            }
        }
    }
    println!("{}", first_line_buf);
    println!("{}", second_line_buf);
    println!("{}", third_line_buf);
    println!();
}

fn part_b(input: &Path) {
    let mut output: i128 = 0;
    let mut map: HashMap<Point, char> = HashMap::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Point> = Vec::new();

    get_numbers_map_and_gears(&mut numbers, &mut map, &mut gears, input);

    let mut neighbours: Vec<i128> = Vec::new();

    for gear in gears {
        for number in numbers.iter() {
            if are_neighbours(&gear, number) {
                neighbours.push(number.value);
            }
        }
        if neighbours.len() == 2 {
            // println!(
            //     "Neighbour 1: {} | Neighbour 2: {} Mult.: {}",
            //     neighbours[0],
            //     neighbours[1],
            //     neighbours[0] * neighbours[1]
            // );
            output += neighbours[0] * neighbours[1];
        } else {
            //pretty_print_gear(&gear, &map)
        }
        neighbours.clear();
    }

    println!("\nOutput 2: {}", output)
}

fn are_neighbours(gear: &Point, number: &Number) -> bool {
    // for point in all_neighbour_points(gear.x, gear.y) {
    //     if point.y == number.start.y && point.x >= number.start.x && point.x <= number.end.x {
    //         return true;
    //     }
    // }
    if gear.x >= number.start.x - 1
        && gear.x <= number.end.x + 1
        && gear.y >= number.start.y - 1
        && gear.y <= number.end.y + 1
    {
        return true;
    }
    false
}

fn get_numbers_map_and_gears(
    numbers: &mut Vec<Number>,
    map: &mut HashMap<Point, char>,
    gears: &mut Vec<Point>,
    input: &Path,
) {
    let mut number: i128 = 0;
    for (y, line) in line_iterator(input).enumerate() {
        // if y < 0 || y >= 5 {
        //     continue;
        // }
        let line = line.expect("Failed to read line");
        //println!("({}): line {}", line, y);
        for (x, character) in line.chars().enumerate() {
            let number_result = get_number(&character, &x, &y, line.len(), &mut number);
            if let Ok(found_number) = number_result {
                numbers.push(found_number);
            } else if character == '*' {
                gears.push(Point {
                    x: x as i128,
                    y: y as i128,
                })
            }
            map.insert(
                Point {
                    x: x as i128,
                    y: y as i128,
                },
                character,
            );
        }
    }
}

fn part_a(input: &Path) {
    let mut output: i128 = 0;
    let mut map: HashMap<Point, char> = HashMap::new();
    let mut numbers: Vec<Number> = Vec::new();

    get_numbers_and_map(&mut numbers, &mut map, input);

    for number in numbers {
        if has_special_neighbour(&number, &map) {
            //println!("{}", number.value.to_string());
            output += number.value;
        }
    }

    println!("\nOutput 1: {}", output)
}

fn get_numbers_and_map(numbers: &mut Vec<Number>, map: &mut HashMap<Point, char>, input: &Path) {
    let mut number: i128 = 0;
    for (y, line) in line_iterator(input).enumerate() {
        let line = line.expect("Failed to read line");
        //println!("({}): line {}", line, y);
        for (x, character) in line.chars().enumerate() {
            let number_result = get_number(&character, &x, &y, line.len(), &mut number);
            if let Ok(found_number) = number_result {
                numbers.push(found_number);
            }
            map.insert(
                Point {
                    x: x as i128,
                    y: y as i128,
                },
                character,
            );
        }
    }
}

fn get_number<'a>(
    character: &char,
    x: &usize,
    y: &usize,
    line_len: usize,
    number: &mut i128,
) -> Result<Number, &'a str> {
    if character.is_ascii_digit() && *x == line_len - 1 {
        *number = *number * 10 + character.to_digit(10).unwrap() as i128;
        let end = Point {
            x: *x as i128,
            y: *y as i128,
        };
        let number_len = number.to_string().len() as i128;
        let start = Point {
            x: *x as i128 - number_len,
            y: *y as i128,
        };
        let output = Number {
            start,
            end,
            value: *number,
        };
        *number = 0;
        return Ok(output);
    } else if character.is_ascii_digit() {
        *number = *number * 10 + character.to_digit(10).unwrap() as i128;
    } else if !character.is_ascii_digit() && *number != 0 {
        let end = Point {
            x: *x as i128 - 1,
            y: *y as i128,
        };
        let number_len = number.to_string().len() as i128;
        let start = Point {
            x: *x as i128 - number_len,
            y: *y as i128,
        };
        let output = Number {
            start,
            end,
            value: *number,
        };
        *number = 0;
        return Ok(output);
    }
    Err("Do nothing.")
}

fn has_special_neighbour(number: &Number, map: &HashMap<Point, char>) -> bool {
    let start = number.start.x;
    let end = number.end.x;
    let y = number.start.y;
    for i in start..=end {
        for neighbour_point in all_neighbour_points(i, y) {
            let neighbour_char_result = map.get(&neighbour_point);
            let neighbour_char = neighbour_char_result.unwrap_or(&'0');
            if is_special_char(neighbour_char) {
                return true;
            }
        }
    }
    false
}

fn all_neighbour_points(x: i128, y: i128) -> [Point; 8] {
    [
        Point { x: x - 1, y: y - 1 },
        Point { x: x - 1, y },
        Point { x: x - 1, y: y + 1 },
        Point { x, y: y - 1 },
        Point { x, y: y + 1 },
        Point { x: x + 1, y: y - 1 },
        Point { x: x + 1, y },
        Point { x: x + 1, y: y + 1 },
    ]
}

fn is_special_char(char_at_p: &char) -> bool {
    if ['+', '/', '=', '*', '@', '%', '-', '#', '&', '$'].contains(char_at_p) {
        return true;
    }
    false
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Number {
    start: Point,
    end: Point,
    value: i128,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i128,
    y: i128,
}
