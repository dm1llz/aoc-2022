use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_first_solution() {
    let calories = read_file();

    println!("Most calories: {}", get_calories_as_vec(&calories)[0]);
}

pub fn get_second_solution() {
    let calories = read_file();
    let calories_vec = get_calories_as_vec(&calories);

    println!("Top 3 total calories: {}", calories_vec[0] + calories_vec[1] + calories_vec[2]);
}

fn get_calories_as_vec(calories: &str) -> Vec<usize> {
    let mut current_calories = 0;
    let mut calories_as_usize_vec = Vec::new();

    for cal in calories.split("\n") {
        if cal.is_empty() {
            calories_as_usize_vec.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += cal.parse::<usize>().unwrap_or(0);
        }
    };

    calories_as_usize_vec.sort_by(|a, b| b.cmp(a));
    calories_as_usize_vec
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