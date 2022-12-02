use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_first_solution() {
    println!("Most calories: {}", get_most_calories());
}

fn get_most_calories() -> usize {
    let calories = read_file();

    let mut most_calories: usize = 0;
    let mut current_calories: usize = 0;

    for cal in calories.split("\n") {
        if cal == "" {
            if current_calories > most_calories {
                most_calories = current_calories;
            }
            current_calories = 0;
        } else {
            let cal_as_usize = match cal.parse::<usize>() {
                Err(_) => 0,
                Ok(val) => val
            };
            current_calories = current_calories + cal_as_usize;
        }
    }

    most_calories
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
        _ => contents,
    }
}