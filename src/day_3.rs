use crate::read_file;
use std::collections::HashSet;

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
        _ => None
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
