use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_8;

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
    day_3::get_second_solution();
    println!("--------------------------------------");

    println!("Day 4");
    day_4::get_first_solution();
    day_4::get_second_solution();
    println!("--------------------------------------");

    println!("Day 5");
    day_5::get_first_solution();
    day_5::get_second_solution();
    println!("--------------------------------------");

    println!("Day 6");
    day_6::get_first_solution();
    day_6::get_second_solution();
    println!("--------------------------------------");

    println!("Day 8");
    day_8::get_first_solution();
    day_8::get_second_solution();
    println!("--------------------------------------");
}

pub fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let mut file = match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("Couldn't read {}: {}", path.display(), why),
        _ => contents,
    }
}
