use std::cmp;
use std::io;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    row: i64,
    col: i64,
}

fn parse_input() -> Vec<Point> {
    let mut points = Vec::<Point>::new();

    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Could not read line");

        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            break;
        } else {
            let (row_str, col_str) = trimmed_line.split_once(",").expect("Invalid point");

            points.push(Point {
                row: row_str.parse().expect("Invalid row"),
                col: col_str.parse().expect("Invalid col"),
            })
        }
    }

    points
}

fn main() {
    let mut max_area: i64 = -1;
    let points = parse_input();

    for first in &points {
        for second in &points {
            if first != second {
                max_area = cmp::max(
                    max_area,
                    ((first.row - second.row + 1) * (first.col - second.col + 1)).abs(),
                );
            }
        }
    }

    println!("Max area {}", max_area);
}
