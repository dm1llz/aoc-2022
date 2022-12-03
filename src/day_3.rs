use crate::read_file;
use std::collections::HashMap;

pub fn get_first_solution() {
    let items = read_file("src/input/day3.txt");

    println!(
        "Priorities: {}",
        items
            .split('\n')
            .map(|i| {
                let common_char = get_common_char(i);

                match common_char {
                    Some(x) => get_value_from_char(x),
                    _ => 0,
                }
            })
            .sum::<usize>()
    );
}

fn get_common_char(rucksack: &str) -> Option<char> {
    let middle = rucksack.len() / 2;
    let mut compartment_1 = HashMap::new();

    for (i, letter) in rucksack.chars().enumerate() {
        if i < middle {
            compartment_1.insert(letter, true);
        } else {
            match compartment_1.contains_key(&letter) {
                true => return Some(letter),
                false => (),
            };
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
