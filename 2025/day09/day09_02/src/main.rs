use day09_02::point::Point;
use day09_02::publish::publish_shape;
use day09_02::shape::*;

use std::io;

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
                x: col_str.parse().expect("Invalid row"),
                y: row_str.parse().expect("Invalid col"),
            })
        }
    }

    points
}

fn main() {
    let mut max_area: i64 = -1;
    let points: Vec<Point> = parse_input();
    let shape = simplify_shape(&points);
    let mut best_point1 = Point { x: 0, y: 0 };
    let mut best_point2 = Point { x: 0, y: 0 };

    for (first_index, first_point) in points.iter().enumerate() {
        for (second_index, second_point) in points.iter().enumerate() {
            if second_index >= first_index {
                break;
            }
            let area = ((second_point.y - first_point.y).abs() + 1)
                * ((second_point.x - first_point.x).abs() + 1);

            if area <= max_area {
                continue;
            }
            let rect = make_rect(first_point, second_point);
            // println!("Rect is {:?}", rect);

            if convex_in_concave_incl(&rect, &shape) {
                max_area = area;
                best_point1 = *first_point;
                best_point2 = *second_point;
                println!("Best area now {}", max_area);
            }
        }
    }

    println!("Max area {}", max_area);
    println!("Points {:?} {:?}", best_point1, best_point2);

    publish_shape(
        "drawing.svg",
        &shape,
        &make_rect(&best_point1, &best_point2),
    );
}

// Too low: 159180048
// Wrong: 1560863364
// Too high: 3449890980
//
// Too high: 4749929916
