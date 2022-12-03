use crate::read_file;
use std::collections::HashMap;

pub fn get_first_solution() {
    let items = read_file("src/input/day3.txt");

    println!(
        "Priorities: {}",
        items
            .split('\n')
            .map(|i| {
                let middle = i.len() / 2;
                let common_char = get_common_char(vec![&i[0..middle], &i[middle..]]);

                match common_char {
                    Some(x) => get_value_from_char(x),
                    _ => 0,
                }
            })
            .sum::<usize>()
    );
}

pub fn get_second_solution() {
    let items = read_file("src/input/day3.txt");
    let mut running_total = 0;
    let mut process_items = Vec::new();

    for item in items.split('\n') {
        if process_items.len() == 3 {
            let common_char = get_common_char(process_items.to_vec());
            running_total += match common_char {
                Some(x) => get_value_from_char(x),
                _ => 0,
            };

            process_items.clear();
        }

        process_items.push(item);
    }

    println!("Group Priorities: {}", running_total);
}

fn get_common_char(rucksacks: Vec<&str>) -> Option<char> {
    let mut common_chars = HashMap::new();
    let mut parsed_chars = HashMap::new();

    for rucksack in rucksacks.iter() {
        for letter in rucksack.chars() {
            match parsed_chars.contains_key(&letter) {
                true => (),
                false => {
                    *common_chars.entry(letter).or_insert(0) += 1;

                    parsed_chars.insert(letter, ());
                }
            };
        }

        parsed_chars = HashMap::new();
    }

    for (key, value) in common_chars {
        if value == rucksacks.len() {
            return Some(key);
        }
    }

    None
}

fn get_value_from_char(letter: char) -> usize {
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    letters.iter().position(|&l| l == letter).unwrap_or(0) + 1
}
