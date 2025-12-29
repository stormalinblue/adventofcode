use std::collections::HashMap;
use std::io;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input() -> Vec<Direction> {
    let mut line_buf = String::new();
    io::stdin()
        .read_line(&mut line_buf)
        .expect("Could not read line");

    line_buf
        .trim()
        .chars()
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("Invalid direction"),
        })
        .collect()
}

fn part1(dirs: &[Direction]) -> i64 {
    let mut current_pos = (0, 0);
    let mut position_count = HashMap::<(i64, i64), i64>::new();
    position_count.insert(current_pos, 1);

    for dir in dirs.iter() {
        let (curr_x, curr_y) = current_pos;
        let next_pos = match dir {
            Direction::Up => (curr_x, curr_y + 1),
            Direction::Down => (curr_x, curr_y - 1),
            Direction::Left => (curr_x - 1, curr_y),
            Direction::Right => (curr_x + 1, curr_y),
        };
        position_count
            .entry(next_pos)
            .and_modify(|x| *x += 1)
            .or_insert(1);
        current_pos = next_pos;
    }

    position_count.len() as i64
}

fn part2(dirs: &[Direction]) -> i64 {
    let mut santa_pos = (0, 0);
    let mut robot_pos = (0, 0);
    let mut position_count = HashMap::<(i64, i64), i64>::new();
    position_count.insert(santa_pos, 2);

    for (index, dir) in dirs.iter().enumerate() {
        let turn = index % 2;
        let (curr_x, curr_y) = if turn == 0 { santa_pos } else { robot_pos };
        let next_pos = match dir {
            Direction::Up => (curr_x, curr_y + 1),
            Direction::Down => (curr_x, curr_y - 1),
            Direction::Left => (curr_x - 1, curr_y),
            Direction::Right => (curr_x + 1, curr_y),
        };
        position_count
            .entry(next_pos)
            .and_modify(|x| *x += 1)
            .or_insert(1);
        if turn == 0 {
            santa_pos = next_pos;
        } else {
            robot_pos = next_pos;
        }
    }

    position_count.len() as i64
}

fn main() {
    let input = parse_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
