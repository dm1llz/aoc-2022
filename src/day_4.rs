use crate::read_file;
use std::collections::HashSet;

pub fn get_first_solution() {
    let ids = read_file("src/input/day4.txt");

    let overlaps: usize = ids
        .split('\n')
        .map(|id| {
            let (pair_1_range, pair_2_range) = get_ranges(id);
            if pair_1_range.0 | pair_1_range.1 | pair_2_range.0 | pair_2_range.1 == -1 {
                return 0;
            }

            let is_overlap = (pair_1_range.0 <= pair_2_range.0 && pair_1_range.1 >= pair_2_range.1)
                || (pair_1_range.0 >= pair_2_range.0 && pair_1_range.1 <= pair_2_range.1);

            match is_overlap {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("Total Overlaps: {}", overlaps);
}

pub fn get_second_solution() {
    let ids = read_file("src/input/day4.txt");

    let overlaps: usize = ids
        .split('\n')
        .map(|id| {
            let (pair_1_range, pair_2_range) = get_ranges(id);

            let numbers: HashSet<i32> = HashSet::from_iter(pair_1_range.0..=pair_1_range.1);
            if numbers.contains(&-1) {
                return 0;
            }

            for num in pair_2_range.0..=pair_2_range.1 {
                if numbers.contains(&num) {
                    return 1;
                }
            }

            0
        })
        .sum();

    println!("Total Overlaps: {}", overlaps);
}

fn get_ranges(pairs_str: &str) -> ((i32, i32), (i32, i32)) {
    let pairs = pairs_str.split(',').collect::<Vec<&str>>();
    if pairs.len() < 2 {
        return ((-1, -1), (-1, -1));
    }
    let pair_1_range = get_pair_start_and_end(pairs[0]);
    let pair_2_range = get_pair_start_and_end(pairs[1]);

    (pair_1_range, pair_2_range)
}

fn get_pair_start_and_end(pair: &str) -> (i32, i32) {
    let divider_index = pair.chars().position(|c| c == '-').unwrap_or(0);
    if divider_index == 0 {
        return (-1, -1);
    }

    (
        pair[..divider_index].parse::<i32>().unwrap_or(-1),
        pair[divider_index + 1..].parse::<i32>().unwrap_or(-1),
    )
}
