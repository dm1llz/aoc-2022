use crate::read_file;

pub fn get_first_solution() {
    let ids = read_file("src/input/day4.txt");

    let overlaps: usize = ids
        .split('\n')
        .map(|id| {
            let pairs = id.split(',').collect::<Vec<&str>>();
            if pairs.len() < 2 {
                return 0;
            }
            let (pair_1_start, pair_1_end) = get_pair_start_and_end(pairs[0]);
            let (pair_2_start, pair_2_end) = get_pair_start_and_end(&pairs[1]);
            let mut is_overlap = (pair_1_start <= pair_2_start && pair_1_end >= pair_2_end)
                || (pair_1_start >= pair_2_start && pair_1_end <= pair_2_end);

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
