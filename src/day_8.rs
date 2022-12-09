pub fn get_first_solution() {
    let grid = crate::read_file("src/input/day8.txt");
    let (rows, columns) = build_rows_and_columns(&grid);
    let mut visible_trees = 0;

    for (i, row) in rows.iter().enumerate() {
        if i == 0 || i == rows.len() - 1 {
            visible_trees += row.len();
            continue;
        }

        for (j, c) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                visible_trees += 1;
                continue;
            }

            let mut current = row[..j].to_vec();
            let mut visibility = get_visibility_score(c, &current);
            if (current.len() == 1 && current.first().unwrap() < c)
                || (visibility == current.len() && current.last().unwrap() < c)
            {
                visible_trees += 1;
                continue;
            }

            current = row[j + 1..].to_vec();
            visibility = get_visibility_score(c, &current);
            if (current.len() == 1 && current.first().unwrap() < c)
                || (visibility == current.len() && current.last().unwrap() < c)
            {
                visible_trees += 1;
                continue;
            }

            current = columns[j][..i].to_vec();
            visibility = get_visibility_score(c, &current);
            if (current.len() == 1 && current.first().unwrap() < c)
                || (visibility == current.len() && current.last().unwrap() < c)
            {
                visible_trees += 1;
                continue;
            }

            current = columns[j][i + 1..].to_vec();
            visibility = get_visibility_score(c, &current);
            if (current.len() == 1 && current.first().unwrap() < c)
                || (visibility == current.len() && current.last().unwrap() < c)
            {
                visible_trees += 1;
                continue;
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
}

pub fn get_second_solution() {
    let grid = crate::read_file("src/input/day8.txt");
    let (rows, columns) = build_rows_and_columns(&grid);
    let mut highest_visibility = 0;

    for (i, row) in rows.iter().enumerate() {
        if i == 0 || i == rows.len() - 1 {
            continue;
        }

        for (j, c) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                continue;
            }

            let visibility_left =
                get_visibility_score(c, &row[..j].iter().rev().copied().collect());
            if visibility_left == 0 {
                continue;
            }

            let visibility_right = get_visibility_score(c, &row[j + 1..].to_vec());
            if visibility_right == 0 {
                continue;
            }

            let visibility_top =
                get_visibility_score(c, &columns[j][..i].iter().rev().copied().collect());
            if visibility_top == 0 {
                continue;
            }

            let visibility_bottom = get_visibility_score(c, &columns[j][i + 1..].to_vec());
            if visibility_bottom == 0 {
                continue;
            }

            let visibility_score =
                visibility_left * visibility_right * visibility_top * visibility_bottom;
            if visibility_score > highest_visibility {
                highest_visibility = visibility_score;
            }
        }
    }

    println!("Highest visibility score: {}", highest_visibility);
}

fn build_rows_and_columns(grid: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut columns: Vec<Vec<usize>> = Vec::new();
    let mut rows = Vec::new();

    for line in grid.lines() {
        let chars: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        for (i, c) in chars.iter().enumerate() {
            match columns.get_mut(i) {
                Some(v) => v.push(*c),
                None => columns.insert(i, vec![*c]),
            };
        }
        rows.push(chars);
    }

    (rows, columns)
}

fn get_visibility_score(tree_size: &usize, trees: &Vec<usize>) -> usize {
    let mut count = 0;

    for c in trees {
        let temp_tree: usize = c.to_string().parse().unwrap();
        count += 1;
        if &temp_tree >= tree_size {
            break;
        }
    }

    count
}
