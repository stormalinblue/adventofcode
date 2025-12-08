use std::io;

fn parse_input() -> Vec<Vec<i64>> {
    let mut matrix: Vec<Vec<i64>> = Vec::new();

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            break;
        } else {
            let matrix_row = trimmed_line
                .chars()
                .map(|x| match x {
                    '@' => 1,
                    _ => 0,
                })
                .collect();
            matrix.push(matrix_row);
        }
    }

    matrix
}

fn main() {
    let mut matrix = parse_input();

    let mut total_movable: i64 = 0;

    loop {
        let mut next_matrix = matrix.clone();

        let num_rows = matrix.len();
        let num_cols = matrix[0].len();

        let mut num_movable = 0;
        for row_index in 0..num_rows {
            for col_index in 0..num_cols {
                if matrix[row_index][col_index] == 0 {
                    continue;
                }

                let mut count: i64 = 0;

                let row_signed = row_index as i64;
                let col_signed = col_index as i64;

                for other_row in (row_signed - 1)..=(row_signed + 1) {
                    for other_col in (col_signed - 1)..=(col_signed + 1) {
                        if other_row == row_signed && other_col == col_signed {
                            continue;
                        }
                        if (0 <= other_row) && (other_row < (num_rows as i64)) {
                            if (0 <= other_col) && (other_col < (num_cols as i64)) {
                                // println!("other row col {} {}", other_row, other_col);
                                count += matrix[other_row as usize][other_col as usize];
                            }
                        }
                    }
                }

                if count < 4 {
                    // println!("movable {} {}", row_index, col_index);
                    num_movable += 1;
                    next_matrix[row_index as usize][col_index as usize] = 0;
                }
            }
        }

        // println!("matrix {:?}", matrix);
        // println!("counts {:?}", count_matrix);
        // println!("num movable {}", num_movable);

        if num_movable == 0 {
            break;
        } else {
            total_movable += num_movable;
            matrix = next_matrix;
        }
    }

    println!("Total movable: {}", total_movable);
}
