use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod day_1;
mod day_2;
mod day_3;

fn main() {
    println!("Day 1");
    day_1::get_first_solution();
    day_1::get_second_solution();
    println!("--------------------------------------");

    println!("Day 2");
    day_2::get_first_solution();
    day_2::get_second_solution();
    println!("--------------------------------------");

    println!("Day 3");
    day_3::get_first_solution();
    println!("--------------------------------------");
}

pub fn read_file(path: &str) -> Vec<&str> {
    let path = Path::new(path);
    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        _ => contents.split('\n').collect(),
    }
}
