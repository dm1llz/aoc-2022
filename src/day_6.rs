pub fn get_first_solution() {
    let buffer = crate::read_file("src/input/day6.txt");

    println!("First start signal index: {}", get_signal_start(&buffer, 4));
}

pub fn get_second_solution() {
    let buffer = crate::read_file("src/input/day6.txt");

    println!(
        "First start of message signal index: {}",
        get_signal_start(&buffer, 14)
    );
}

fn get_signal_start(buffer: &str, length: usize) -> i32 {
    let mut parsed = String::new();

    for (i, character) in buffer.chars().enumerate() {
        if parsed.contains(character) {
            for (j, c) in parsed.chars().enumerate() {
                if c == character {
                    parsed = String::from(&parsed[j + 1..]);
                    break;
                }
            }
        }

        parsed += &character.to_string();

        if parsed.len() == length {
            return i as i32 + 1;
        }
    }

    -1
}
