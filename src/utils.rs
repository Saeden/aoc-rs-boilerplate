use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn line_iterator(path: &Path) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Could not read line"))
        .collect()
}
