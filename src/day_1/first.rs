use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_solution() -> usize {
    println!("File contents {}", read_file());
}

fn read_file() -> String {
    let path = Path::new("src/day_1/input.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        _ => contents
    }
}

fn get_most_calories(calories: &str) -> usize {
    0
}