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
    let matrix = parse_input();

    let mut count_matrix = matrix.clone();

    for row in count_matrix.iter_mut() {
        for item in row.iter_mut() {
            *item = 0;
        }
    }

    let num_rows = matrix.len() as i64;
    let num_cols = matrix[0].len() as i64;

    let mut num_movable = 0;
    for row_index in 0..num_rows {
        for col_index in 0..num_cols {
            if matrix[row_index as usize][col_index as usize] == 0 {
                continue;
            }

            for other_row in (row_index - 1)..=(row_index + 1) {
                for other_col in (col_index - 1)..=(col_index + 1) {
                    if other_row == row_index && other_col == col_index {
                        continue;
                    }
                    if (0 <= other_row) && (other_row < num_rows) {
                        if (0 <= other_col) && (other_col < num_cols) {
                            // println!("other row col {} {}", other_row, other_col);
                            count_matrix[row_index as usize][col_index as usize] +=
                                matrix[other_row as usize][other_col as usize];
                        }
                    }
                }
            }

            if count_matrix[row_index as usize][col_index as usize] < 4 {
                // println!("movable {} {}", row_index, col_index);
                num_movable += 1;
            }
        }
    }

    // println!("matrix {:?}", matrix);
    // println!("counts {:?}", count_matrix);
    println!("num movable {}", num_movable);
}
