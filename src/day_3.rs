use crate::read_file;
use std::collections::HashSet;

pub fn get_first_solution() {
    let items = read_file("src/input/day3.txt");

    let total: usize = items
        .split('\n')
        .map(|i| {
            let middle = i.len() / 2;

            match get_common_char(vec![&i[0..middle], &i[middle..]]) {
                Some(c) => get_value_from_char(c),
                None => 0,
            }
        })
        .sum();

    println!("Priorities: {}", total);
}

pub fn get_second_solution() {
    let items = read_file("src/input/day3.txt");

    let total: usize = items
        .split('\n')
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|c| match get_common_char(c.to_vec()) {
            Some(c) => get_value_from_char(c),
            None => 0,
        })
        .sum();

    println!("Group Priorities: {}", total);
}

fn get_common_char(rucksacks: Vec<&str>) -> Option<char> {
    if rucksacks.is_empty() {
        return None;
    }

    let mut common_chars: HashSet<char> = HashSet::from_iter(rucksacks[0].chars());

    for rucksack in rucksacks.iter().skip(1) {
        let rucksack_chars = HashSet::from_iter(rucksack.chars());
        let mut intersects = HashSet::new();

        for intersect in common_chars.intersection(&rucksack_chars) {
            intersects.insert(*intersect);
        }

        common_chars = intersects;
    }

    match common_chars.len() {
        1 => common_chars.iter().next().copied(),
        _ => None,
    }
}

fn get_value_from_char(letter: char) -> usize {
    let letters = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    letters.iter().position(|&l| l == letter).unwrap_or(0) + 1
}
