pub fn get_first_solution() {
    let moves = crate::read_file("src/input/day2.txt");

    println!("Total score: {}", calc_scores(moves, calc_score));
}

pub fn get_second_solution() {
    let moves = crate::read_file("src/input/day2.txt");

    println!("Total score: {}", calc_scores(moves, find_move_then_calc));
}

fn calc_scores(moves: Vec<&str>, func: fn(char, char) -> usize) -> usize {
    moves
        .into_iter()
        .map(|m| {
            if !m.is_empty() {
                let move_split: Vec<&str> = m.split(' ').collect();
                func(
                    move_split[0].parse::<char>().unwrap(),
                    move_split[1].parse::<char>().unwrap(),
                )
            } else {
                0
            }
        })
        .sum()
}

fn calc_score(their_move: char, my_move: char) -> usize {
    let move_score = match my_move {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    let round_result = if (my_move == 'X' && their_move == 'C')
        || (my_move == 'Y' && their_move == 'A')
        || (my_move == 'Z' && their_move == 'B')
    {
        6
    } else if (my_move == 'X' && their_move == 'B')
        || (my_move == 'Y' && their_move == 'C')
        || (my_move == 'Z' && their_move == 'A')
    {
        0
    } else {
        3
    };

    move_score + round_result
}

fn find_move_then_calc(their_move: char, result: char) -> usize {
    let my_move = if result == 'X' {
        match their_move {
            'A' => 'Z',
            'B' => 'X',
            'C' => 'Y',
            _ => 'U',
        }
    } else if result == 'Y' {
        match their_move {
            'A' => 'X',
            'B' => 'Y',
            'C' => 'Z',
            _ => 'U',
        }
    } else {
        match their_move {
            'A' => 'Y',
            'B' => 'Z',
            'C' => 'X',
            _ => 'U',
        }
    };

    calc_score(their_move, my_move)
}
