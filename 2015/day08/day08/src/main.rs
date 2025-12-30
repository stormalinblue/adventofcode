use std::io::{self, BufRead};

fn parse_input() -> Vec<String> {
    let mut result = Vec::<String>::new();

    for line_result in io::stdin().lock().lines() {
        let line = line_result.expect("Could not read line").trim().to_string();

        result.push(line);
    }

    result
}

fn in_memory_size(string: &str) -> usize {
    let mut started = false;
    let mut escaped = false;
    let mut count: usize = 0;
    let mut x_seq_count: usize = 0;
    for char in string.chars() {
        match char {
            '"' => {
                if !started {
                    started = true;
                } else if escaped {
                    count += 1;
                    escaped = false;
                } else {
                    // should be at the end of the string
                    break;
                }
            }
            '\\' => {
                if !escaped {
                    escaped = true;
                } else {
                    count += 1;
                    escaped = false;
                }
            }
            'x' => {
                if !escaped {
                    count += 1;
                } else {
                    x_seq_count = 1;
                }
            }
            _ => match x_seq_count {
                1 => x_seq_count = 2,
                2 => {
                    x_seq_count = 0;
                    escaped = false;
                    count += 1;
                }
                0 => {
                    if escaped {
                        count += 1;
                        escaped = false;
                    } else {
                        count += 1;
                    }
                }
                _ => unreachable!(),
            },
        }
    }
    count
}

fn part1(strings: &[String]) -> i64 {
    strings
        .iter()
        .map(|x| x.len() - in_memory_size(x.as_str()))
        .sum::<usize>() as i64
}

fn part2(strings: &[String]) -> i64 {
    strings
        .iter()
        .map(|x| x.escape_default().count() + 2 - x.len())
        .sum::<usize>() as i64
}

fn main() {
    let input = parse_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
