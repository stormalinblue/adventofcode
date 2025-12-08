use std::{io, ops::Index};

enum Operation {
    Mul,
    Add,
}

fn main() {
    let op_row: Vec<Operation>;
    let mut operand_rows: Vec<Vec<i64>> = Vec::new();

    loop {
        let mut row_line = String::new();
        io::stdin()
            .read_line(&mut row_line)
            .expect("Could not read line.");

        if row_line.contains("+") || row_line.contains("*") {
            op_row = row_line
                .split_whitespace()
                .map(|x| match x {
                    "+" => Operation::Add,
                    "*" => Operation::Mul,
                    _ => panic!(),
                })
                .collect();
            break;
        } else {
            let num_row = row_line
                .split_whitespace()
                .map(|x| x.parse::<i64>().expect("Invalid int"))
                .collect();
            operand_rows.push(num_row);
        }
    }

    println!("num lines {}", operand_rows.len());

    let mut result_sum: i64 = 0;
    for (col_index, column_op) in op_row.iter().enumerate() {
        match column_op {
            Operation::Add => {
                let result: i64 = operand_rows
                    .iter()
                    .map(|a| *a.index(col_index))
                    .reduce(|a, b| a + b)
                    .unwrap_or(1);
                result_sum += result;
            }
            Operation::Mul => {
                let result: i64 = operand_rows
                    .iter()
                    .map(|a| *a.index(col_index))
                    .reduce(|a, b| a * b)
                    .unwrap_or(1);
                result_sum += result;
            }
        }
    }

    println!("Result sum {}", result_sum)
}
