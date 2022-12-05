#[derive(PartialEq)]
enum ArrangeType {
    Group,
    Single,
}

pub fn get_first_solution() {
    let input = crate::read_file("src/input/day5.txt");

    let mut stacks = get_stacks(input.split('\n').take(8).collect());
    arrange_stacks(
        &mut stacks,
        input.split('\n').skip(10).collect(),
        ArrangeType::Single,
    );

    println!("Top crates: {}", get_top_crates(stacks));
}

pub fn get_second_solution() {
    let input = crate::read_file("src/input/day5.txt");

    let mut stacks = get_stacks(input.split('\n').take(8).collect());
    arrange_stacks(
        &mut stacks,
        input.split('\n').skip(10).collect(),
        ArrangeType::Group,
    );

    println!("Top crates: {}", get_top_crates(stacks));
}

fn get_stacks(input: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];

    for stack in input {
        for (i, c) in stack.chars().collect::<Vec<char>>().chunks(4).enumerate() {
            if !c.is_empty() && c[1] != ' ' {
                stacks[i].insert(0, c[1]);
            }
        }
    }

    stacks
}

fn arrange_stacks(stacks: &mut Vec<Vec<char>>, moves: Vec<&str>, arrange: ArrangeType) {
    for instruction in moves {
        let parts: Vec<&str> = instruction.split(' ').collect();
        if parts.len() > 1 {
            let count = parts[1].parse::<usize>().unwrap_or(0);
            let initial_stack = parts[3].parse::<usize>().unwrap_or(0) - 1;
            let new_stack = parts[5].parse::<usize>().unwrap_or(0) - 1;

            let mut moved_stack = Vec::new();
            for _ in 0..count {
                let top_crate = stacks[initial_stack].pop();
                if let Some(c) = top_crate {
                    moved_stack.push(c);
                }
            }

            if arrange == ArrangeType::Group {
                moved_stack = moved_stack.iter().rev().copied().collect();
            }

            for c in moved_stack {
                stacks[new_stack].push(c);
            }
        }
    }
}

fn get_top_crates(stacks: Vec<Vec<char>>) -> String {
    let mut top_crates = String::new();

    for mut stack in stacks {
        if let Some(c) = stack.pop() {
            top_crates += &c.to_string();
        }
    }

    top_crates
}
