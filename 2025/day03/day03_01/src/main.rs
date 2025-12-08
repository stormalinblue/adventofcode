use std::{cmp, io};

fn parse_input() -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    loop {
        let mut row_str = String::new();
        io::stdin()
            .read_line(&mut row_str)
            .expect("Could not read line");

        let trimmed_row = row_str.trim();

        if trimmed_row.is_empty() {
            break;
        } else {
            result.push(
                trimmed_row
                    .chars()
                    .map(|c| (c as i32) - ('0' as i32))
                    .collect(),
            )
        }
    }

    result
}

fn joltage(row: &[i32]) -> i32 {
    let mut best_after: Vec<i32> = Vec::new();

    let mut running_max = -1;
    for item in row.iter().rev() {
        running_max = cmp::max(running_max, *item);
        best_after.push(running_max);
    }

    best_after.reverse();

    let mut best_joltage: i32 = -1;
    for (index, first) in row[0..row.len() - 1].iter().enumerate() {
        let second = best_after[index + 1];

        let current_joltage = first * 10 + second;

        best_joltage = cmp::max(current_joltage, best_joltage);
    }

    best_joltage
}

fn main() {
    let total_joltage = parse_input()
        .iter()
        .map(|x| joltage(x.as_slice()))
        .sum::<i32>();

    println!("Total joltage: {}", total_joltage);
}
