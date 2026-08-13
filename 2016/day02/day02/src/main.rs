use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

#[derive(Debug)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input() -> Vec<Vec<Instruction>> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .trim()
                .chars()
                .map(|c| match c {
                    'U' => Instruction::Up,
                    'D' => Instruction::Down,
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => panic!("Unknown instruction"),
                })
                .collect::<Vec<Instruction>>()
        })
        .collect()
}

fn part1(input: &Vec<Vec<Instruction>>) -> String {
    let mut result = String::new();
    let pad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut coords = [1i64, 1];
    for digit_instr in input.iter() {
        for instr in digit_instr.iter() {
            let next_coords = match instr {
                Instruction::Up => [max(coords[0] - 1, 0), coords[1]],
                Instruction::Down => [min(coords[0] + 1, 2), coords[1]],
                Instruction::Left => [coords[0], max(coords[1] - 1, 0)],
                Instruction::Right => [coords[0], min(coords[1] + 1, 2)],
            };
            coords = next_coords;
        }
        result.push(pad[coords[0] as usize][coords[1] as usize]);
    }

    result
}

fn part2(input: &Vec<Vec<Instruction>>) -> String {
    let pad = [
        [None, None, Some('1'), None, None],
        [None, Some('2'), Some('3'), Some('4'), None],
        [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        [None, Some('A'), Some('B'), Some('C'), None],
        [None, None, Some('D'), None, None],
    ];

    let mut result = String::new();
    let mut coords = [2i64, 0];

    let at = |coord: &[i64; 2]| -> Option<char> { pad[coord[0] as usize][coord[1] as usize] };

    for digit_instr in input.iter() {
        for instr in digit_instr.iter() {
            let next_coords = match instr {
                Instruction::Up => [max(coords[0] - 1, 0), coords[1]],
                Instruction::Down => [min(coords[0] + 1, 4), coords[1]],
                Instruction::Left => [coords[0], max(coords[1] - 1, 0)],
                Instruction::Right => [coords[0], min(coords[1] + 1, 4)],
            };

            if let Some(_) = at(&next_coords) {
                coords = next_coords;
            }
        }
        result.push(at(&coords).expect("Should remain in a real digit"));
    }

    result
}

fn main() {
    let input = parse_input();
    println!("Input {:?}", input);

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
