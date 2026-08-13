use std::io::{self, Read};

type Vector = [i64; 2];

fn turn_left(input: &Vector) -> Vector {
    let [x, y] = input;

    [-y, *x]
}

fn turn_right(input: &Vector) -> Vector {
    let [x, y] = input;

    [*y, -x]
}

fn move_cursor(source: &Vector, dir: &Vector, mul: i64) -> Vector {
    let [sx, sy] = source;
    let [dx, dy] = dir;

    [sx + dx * mul, sy + dy * mul]
}

fn l1_norm(vec: &Vector) -> i64 {
    let [x, y] = vec;
    x.abs() + y.abs()
}

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: TurnDirection,
    distance: i64,
}

fn parse_input() -> Vec<Instruction> {
    let input_str = {
        let mut buf: String = String::new();
        io::stdin()
            .lock()
            .read_to_string(&mut buf)
            .expect("Could not read string");
        buf
    };

    input_str
        .trim()
        .split(", ")
        .map(|word| {
            let (turn_str, dist_str) = word.split_at(1);
            let direction = match turn_str {
                "R" => TurnDirection::Right,
                "L" => TurnDirection::Left,
                _ => panic!("Expected R or L"),
            };

            let distance = dist_str.parse().expect("Expected number");
            Instruction {
                direction,
                distance,
            }
        })
        .collect()
}

fn part1(instructions: &Vec<Instruction>) -> i64 {
    let mut position = [0, 0];
    let mut direction = [0, 1];

    for instr in instructions.iter() {
        direction = match instr.direction {
            TurnDirection::Left => turn_left(&direction),
            TurnDirection::Right => turn_right(&direction),
        };

        position = move_cursor(&position, &direction, instr.distance);
    }

    l1_norm(&position)
}

fn main() {
    let input = parse_input();

    println!("{}", part1(&input));
}
