use std::io::{self, BufRead};

fn parse_input() -> Vec<String> {
    io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("Could not read line").trim().to_string())
        .collect()
}

fn is_nice_part_1(string: &str) -> bool {
    let mut num_vowels = 0;

    for char in string.chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => num_vowels += 1,
            _ => {}
        }
    }

    if num_vowels < 3 {
        return false;
    }

    let mut has_pair = false;
    for i in 0..(string.len() - 1) {
        match &string[i..i + 2] {
            "ab" | "cd" | "pq" | "xy" => return false,
            _ => {}
        }

        let first = &string[i..i + 1];
        let second = &string[i + 1..i + 2];

        if first == second {
            has_pair = true;
        }
    }

    has_pair
}

fn part1(strings: &[String]) -> i64 {
    let mut num_nice: i64 = 0;

    for s in strings.iter() {
        if is_nice_part_1(s) {
            num_nice += 1;
        }
    }

    num_nice
}

fn is_triple(string: &str) -> bool {
    string[0..1] == string[2..3]
}

fn has_triple(string: &str) -> bool {
    for index in 0..=(string.len() - 3) {
        let slice = &string[index..(index + 3)];

        if is_triple(slice) {
            return true;
        }
    }
    false
}

fn has_repeating_pair(string: &str) -> bool {
    for left_index in 0..=(string.len() - 4) {
        let left_slice = &string[left_index..(left_index + 2)];
        for right_index in (left_index + 2)..=(string.len() - 2) {
            let right_slice = &string[right_index..(right_index + 2)];

            if left_slice == right_slice {
                return true;
            }
        }
    }
    false
}

fn is_nice_part_2(string: &str) -> bool {
    has_triple(string) && has_repeating_pair(string)
}

fn part2(strings: &[String]) -> i64 {
    let mut num_nice: i64 = 0;

    for s in strings.iter() {
        if is_nice_part_2(s) {
            num_nice += 1;
        }
    }

    num_nice
}

fn main() {
    let input = parse_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
