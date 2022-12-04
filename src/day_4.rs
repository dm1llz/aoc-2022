use crate::read_file;

pub fn get_first_solution() {
    let ids = read_file("src/input/day4.txt");

    let overlaps: usize = ids
        .split('\n')
        .map(|id| {
            let (mut pair_1_start, mut pair_1_end) = (-1, -1);
            let mut is_overlap = false;

            for (i, pair) in id.split(',').enumerate() {
                if i == 0 {
                    let (start, end) = get_pair_start_and_end(pair);

                    pair_1_start = start;
                    pair_1_end = end;
                } else {
                    let (start, end) = get_pair_start_and_end(pair);

                    is_overlap = ((pair_1_start <= start && pair_1_end >= end)
                        || (pair_1_start >= start && pair_1_end <= end));
                }
            }

            match is_overlap {
                true => 1,
                false => 0,
            }
        })
        .sum();

    println!("Total Overlaps: {}", overlaps);
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
