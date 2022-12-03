pub fn get_first_solution() {
    let calories = crate::read_file("src/input/day1.txt");

    println!("Most calories: {}", get_calories_as_vec(calories)[0]);
}

pub fn get_second_solution() {
    let calories = crate::read_file("src/input/day1.txt");
    let calories_vec = get_calories_as_vec(calories);

    println!(
        "Top 3 total calories: {}",
        calories_vec[0] + calories_vec[1] + calories_vec[2]
    );
}

fn get_calories_as_vec(calories: Vec<&str>) -> Vec<usize> {
    let mut current_calories = 0;
    let mut calories_as_usize_vec = Vec::new();

    for cal in calories {
        if cal.is_empty() {
            calories_as_usize_vec.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += cal.parse::<usize>().unwrap_or(0);
        }
    }

    calories_as_usize_vec.sort_by(|a, b| b.cmp(a));
    calories_as_usize_vec
}
