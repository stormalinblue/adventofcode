use std::{cmp, io};

fn parse_input() -> Vec<Vec<i64>> {
    let mut result: Vec<Vec<i64>> = Vec::new();

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
                    .map(|c| (c as i64) - ('0' as i64))
                    .collect(),
            )
        }
    }

    result
}

fn joltage(row: &[i64], num_digits: usize) -> Option<i64> {
    let mut best_with_digits: Vec<Option<i64>> = vec![None; num_digits + 1];
    best_with_digits[0] = Some(0);

    for digit in row {
        let mut new_best: Vec<Option<i64>> = vec![None; num_digits + 1];
        new_best[0] = Some(0);

        for (old_num_digits, old_best) in best_with_digits[0..(best_with_digits.len() - 1)]
            .iter()
            .enumerate()
        {
            // println!(
            //     "old num digits {} old best {:?} next {:?}",
            //     old_num_digits,
            //     old_best,
            //     best_with_digits[old_num_digits + 1]
            // );

            new_best[old_num_digits + 1] = match (old_best, best_with_digits[old_num_digits + 1]) {
                (None, None) => break,
                (Some(old_best_val), None) => Some(10 * old_best_val + digit),
                (Some(old_best_val), Some(old_best_full)) => {
                    Some(cmp::max(10 * old_best_val + digit, old_best_full))
                }
                (None, Some(_)) => unreachable!(),
            }
        }

        best_with_digits = new_best;
    }

    *best_with_digits.last().unwrap()
}

fn main() {
    let total_joltage = parse_input()
        .iter()
        .map(|x| joltage(x.as_slice(), 12).unwrap())
        .sum::<i64>();

    println!("Total joltage: {}", total_joltage);
}
