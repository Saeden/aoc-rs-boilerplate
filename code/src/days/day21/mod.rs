use crate::PuzzlePart;
use std::path::Path;
use utils::line_iterator;

pub fn run(part: PuzzlePart, input: &Path) {
    match part {
        PuzzlePart::A => part_a(input),
        PuzzlePart::B => part_b(input),
    }
}

fn part_a(input: &Path) {
    for _line in line_iterator(input) {
        continue;
    }
    unimplemented!()
}

fn part_b(input: &Path) {
    for _line in line_iterator(input) {
        continue;
    }
    unimplemented!()
}
