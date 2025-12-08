use std::io;

#[derive(Debug)]
enum Operation {
    Mul,
    Add,
}

fn main() {
    let op_row: Vec<Operation>;
    let mut operand_rows_raw: Vec<String> = Vec::new();

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
            operand_rows_raw.push(row_line);
        }
    }

    println!("num lines {}", operand_rows_raw.len());

    let max_len = operand_rows_raw.iter().map(|x| x.len()).max().unwrap_or(0);

    for line in operand_rows_raw.iter_mut() {
        line.push_str((" ".repeat(max_len - line.len())).as_str());
    }

    let mut operand_columns: Vec<Vec<i64>> = Vec::new();
    let mut current_operands: Vec<i64> = Vec::new();
    let mut group_open: bool = false;
    for index in (0..max_len).rev() {
        let mut all_empty: bool = true;
        let mut number = 0;
        for line in operand_rows_raw.iter() {
            let char = &line[index..index + 1];

            if !char.trim().is_empty() {
                all_empty = false;
                group_open = true;
                number = number * 10 + char.parse::<i64>().unwrap_or(0);
            }
        }

        if all_empty {
            if group_open {
                group_open = false;
                operand_columns.push(current_operands);
                current_operands = Vec::new();
            }
        } else {
            current_operands.push(number);
        }
    }
    if group_open {
        operand_columns.push(current_operands);
    }
    // println!("Operand columns {:?}", operand_columns);

    operand_columns.reverse();

    let mut result_sum: i64 = 0;
    for (col_index, column_op) in op_row.iter().enumerate() {
        let operands = &operand_columns[col_index];
        // println!("Operation {:?} Operands {:?}", column_op, operands);
        match column_op {
            Operation::Add => {
                let result = operands.iter().sum::<i64>();
                result_sum += result;
            }
            Operation::Mul => {
                let result: i64 = operands.iter().copied().reduce(|a, b| a * b).unwrap_or(1);
                result_sum += result;
            }
        }
    }

    println!("Result sum {}", result_sum)
}
