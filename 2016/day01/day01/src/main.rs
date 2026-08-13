use std::io::{self, Read};

use day01::{
    point::Point,
    two_point::{ClopenSegment, Crosses, TwoPoint},
};

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

#[derive(Debug)]
struct Update {
    src: Vector,
    dst: Vector,
}

fn updates(instructions: &Vec<Instruction>) -> Vec<Update> {
    let mut position = [0, 0];
    let mut direction = [0, 1];

    let mut updates = Vec::new();

    for instr in instructions.iter() {
        direction = match instr.direction {
            TurnDirection::Left => turn_left(&direction),
            TurnDirection::Right => turn_right(&direction),
        };

        let new_position = move_cursor(&position, &direction, instr.distance);
        updates.push(Update {
            src: position,
            dst: new_position,
        });
        position = new_position;
    }

    updates
}

fn part1(update_list: &Vec<Update>) -> i64 {
    l1_norm(&update_list.last().expect("Should have at least one").dst)
}

fn part2(raw_update_list: &Vec<Update>) -> i64 {
    fn from_update(update: &Update) -> ClopenSegment {
        ClopenSegment {
            points: TwoPoint {
                first: Point {
                    x: update.src[0],
                    y: update.src[1],
                },
                second: Point {
                    x: update.dst[0],
                    y: update.dst[1],
                },
            },
        }
    }

    let update_list: Vec<_> = raw_update_list.iter().map(from_update).collect();

    for (latest_index, latest) in update_list.iter().enumerate().skip(1) {
        let mut earliest_intersection = None;
        for (_, prev) in update_list.iter().enumerate().take(latest_index) {
            if (latest, prev).crosses() {
                let intersection_point: Point = if latest.points.first.x == latest.points.second.x {
                    Point {
                        x: latest.points.first.x,
                        y: prev.points.first.y,
                    }
                } else {
                    Point {
                        x: prev.points.first.x,
                        y: latest.points.first.y,
                    }
                };

                let intersection_disp = &intersection_point - &latest.points.first;
                let intersection_time = intersection_disp.x.abs() + intersection_disp.y.abs();
                earliest_intersection = if let Some(prev) = earliest_intersection {
                    let (earliest_time, _) = prev;
                    if intersection_time < earliest_time {
                        Some((intersection_time, intersection_point))
                    } else {
                        Some(prev)
                    }
                } else {
                    Some((intersection_time, intersection_point))
                }
            }
            if let Some((_, intersection_point)) = earliest_intersection {
                return intersection_point.x.abs() + intersection_point.y.abs();
            }
        }
    }
    return 0;
}

fn main() {
    let input = parse_input();

    let update_list = updates(&input);

    println!("{}", part1(&update_list));
    println!("{}", part2(&update_list));
}
