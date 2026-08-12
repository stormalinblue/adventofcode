use std::{
    cmp::{max, min},
    io::{self, BufRead},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OpCode {
    Toggle,
    TurnOff,
    TurnOn,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: OpCode,
    top_left: Point,
    bottom_right: Point,
}

fn parse_input() -> Vec<Instruction> {
    fn parse_point(point_desc: &str) -> Point {
        let [x, y]: [usize; 2] = point_desc
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();

        Point { x, y }
    }

    fn parse_line(line: &str) -> Instruction {
        let words: Vec<_> = line.split_ascii_whitespace().collect();
        let (opcode, [point_1, point_2]) = match words.len() {
            4 => (
                OpCode::Toggle,
                [parse_point(words[1]), parse_point(words[3])],
            ),
            5 => {
                let points = [parse_point(words[2]), parse_point(words[4])];
                match words[1] {
                    "on" => (OpCode::TurnOn, points),
                    _ => (OpCode::TurnOff, points),
                }
            }
            _ => {
                panic!()
            }
        };
        Instruction {
            opcode,
            top_left: Point {
                x: min(point_1.x, point_2.x),
                y: min(point_1.y, point_2.y),
            },
            bottom_right: Point {
                x: max(point_1.x, point_2.x),
                y: max(point_1.y, point_2.y),
            },
        }
    }

    io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(x.expect("Could not read line").as_str()))
        .collect()
}

fn part1(input: &Vec<Instruction>) -> u64 {
    let mut lights: [[u8; 1000]; 1000] = [[0; 1000]; 1000];

    for instr in input.iter() {
        match instr.opcode {
            OpCode::TurnOff => {
                for y in (instr.top_left.y)..=(instr.bottom_right.y) {
                    for x in (instr.top_left.x)..=(instr.bottom_right.x) {
                        lights[y][x] = 0;
                    }
                }
            }
            OpCode::TurnOn => {
                for y in (instr.top_left.y)..=(instr.bottom_right.y) {
                    for x in (instr.top_left.x)..=(instr.bottom_right.x) {
                        lights[y][x] = 1;
                    }
                }
            }
            OpCode::Toggle => {
                for y in (instr.top_left.y)..=(instr.bottom_right.y) {
                    for x in (instr.top_left.x)..=(instr.bottom_right.x) {
                        lights[y][x] ^= 1;
                    }
                }
            }
        }
    }

    // let image_map: String = lights
    //     .iter()
    //     .map(|row| {
    //         row.iter()
    //             .map(|x| match *x {
    //                 0 => '.',
    //                 1 => 'o',
    //                 _ => panic!(),
    //             })
    //             .collect::<String>()
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // println!("{}", image_map);

    lights.iter().flatten().map(|x| *x as u64).sum()
}

fn part2(input: &Vec<Instruction>) -> u64 {
    let mut lights: [[u16; 1000]; 1000] = [[0; 1000]; 1000];

    for instr in input.iter() {
        match instr.opcode {
            OpCode::TurnOff => {
                for y in (instr.top_left.y)..=(instr.bottom_right.y) {
                    for x in (instr.top_left.x)..=(instr.bottom_right.x) {
                        lights[y][x] = max(1, lights[y][x]) - 1;
                    }
                }
            }
            OpCode::TurnOn => {
                for y in (instr.top_left.y)..=(instr.bottom_right.y) {
                    for x in (instr.top_left.x)..=(instr.bottom_right.x) {
                        lights[y][x] += 1;
                    }
                }
            }
            OpCode::Toggle => {
                for y in (instr.top_left.y)..=(instr.bottom_right.y) {
                    for x in (instr.top_left.x)..=(instr.bottom_right.x) {
                        lights[y][x] += 2;
                    }
                }
            }
        }
    }

    lights.iter().flatten().map(|x| *x as u64).sum()
}

fn main() {
    let input = parse_input();

    // println!("{:?}", input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
