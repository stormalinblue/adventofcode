use crate::lib::*;

use std::cmp;
use std::io;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    col: i64,
    row: i64,
}

impl<'b> Sub<&'b Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &'b Self) -> Self::Output {
        Point {col: self.col - rhs.col, row: self.row - rhs.row}
    }
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
            let (col_str, row_str) = trimmed_line.split_once(",").expect("Invalid point");

            points.push(Point {
                row: row_str.parse().expect("Invalid row"),
                col: col_str.parse().expect("Invalid col"),
            })
        }
    }

    points
}

fn in_strict_interior(bound1: &Point, bound2: &Point, test: &Point) -> bool {
    let tr_point = Point {
        row: cmp::min(bound1.row, bound2.row),
        col: cmp::max(bound1.col, bound2.col),
    };
    let bl_point = Point {
        row: cmp::max(bound1.row, bound2.row),
        col: cmp::min(bound1.col, bound2.col),
    };

    return tr_point.row < test.row
        && tr_point.col > test.col
        && bl_point.row > test.row
        && bl_point.col < test.col;
}

fn cross(a: &Point, b: &Point, c: &Point) -> i64 {
    let diff_ab = Point {
        row: a.row - b.row,
        col: a.col - b.col,
    };
    let diff_bc = Point {
        row: b.row - c.row,
        col: b.col - c.col,
    };

    diff_ab.row * diff_bc.col - diff_ab.col * diff_bc.row
}

fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
    cross(a, b, c) > 0
}

fn area(a: &Point, b: &Point) -> i64 {
    return ((a.row - b.row).abs() + 1) * ((a.col - b.col).abs() + 1);
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_ccw() {
        assert!(!ccw(
            &Point { row: -1, col: 0 },
            &Point { row: 0, col: 1 },
            &Point { row: 1, col: 0 }
        ));
    }

    #[test]
    fn test_contain_simple() {
        assert!(!point_inside(
            &Point { row: -1, col: -1 },
            &[
                Point { row: 0, col: 0 },
                Point { row: 1, col: 0 },
                Point { row: 0, col: 1 },
            ],
        ))
    }

    #[test]
    fn test_contain_concave() {
        assert!(!point_inside(
            &Point { row: -1, col: 1 },
            &[
                Point { row: 0, col: 0 },
                Point { row: 5, col: 0 },
                Point { row: 1, col: 1 },
                Point { row: 0, col: 5 },
            ],
        ))
    }
}

fn edges<'a, I>(shape: I) -> impl Iterator<Item = (&'a Point, &'a Point)>
where
    I: IntoIterator<Item = &'a Point>,
{
    shape
        .iter()
        .zip(shape.iter().cycle().skip(1))
        .take(shape.len())
}

fn on_line_segment_inc(point: &Point, line: (&Point, &Point)) -> {
    let left = line.0;
    let right = line.1;


}

fn point_inside(point: &Point, shape: &[Point]) -> bool {
    if shape.iter().any(|x| x == point) {
        return true;
    }

    for (point1, point2) in edges(&shape) {

    }

    // assert!(num_switches % 2 == 0);
    return num_switches % 2 != 0;
}

fn shape_inside(small: &[Point], big: &[Point]) -> bool {
    small.iter().all(|point| {
        let result = point_inside(point, big);
        println!("{:?} inside {}", point, result);
        result
    })
}

fn make_square(bound1: &Point, bound2: &Point) -> Vec<Point> {
    let tr_point = Point {
        row: cmp::min(bound1.row, bound2.row),
        col: cmp::max(bound1.col, bound2.col),
    };
    let bl_point = Point {
        row: cmp::max(bound1.row, bound2.row),
        col: cmp::min(bound1.col, bound2.col),
    };
    let tl_point = Point {
        row: tr_point.row,
        col: bl_point.col,
    };
    let br_point = Point {
        row: bl_point.row,
        col: tr_point.col,
    };

    return vec![tr_point, br_point, bl_point, tl_point];
}

fn main() {
    let mut max_area: i64 = -1;
    let points: Vec<Point> = parse_input();

    for first_point in &points {
        for second_point in &points {
            if first_point != second_point {
                println!("Try {:?} {:?}", first_point, second_point);
                let square = make_square(first_point, second_point);
                if shape_inside(square.as_slice(), points.as_slice()) {
                    let ar = area(first_point, second_point);

                    if ar > max_area {
                        println!("Best area {:?} {:?} = {}", first_point, second_point, ar);
                        max_area = ar;
                    }
                }
            }
        }
    }

    println!("Max area {}", max_area);
}

// Too low: 159180048
// Too high: 3449890980
