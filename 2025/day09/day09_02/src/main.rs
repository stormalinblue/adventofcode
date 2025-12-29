use day09_02::point::Point;
use day09_02::*;

use std::cmp;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::{BufWriter, Write as IOWrite};

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

fn make_rect(bound1: &Point, bound2: &Point) -> Vec<Point> {
    let tr_point = Point {
        x: cmp::min(bound1.x, bound2.x),
        y: cmp::max(bound1.y, bound2.y),
    };
    let bl_point = Point {
        x: cmp::max(bound1.x, bound2.x),
        y: cmp::min(bound1.y, bound2.y),
    };
    let tl_point = Point {
        x: tr_point.x,
        y: bl_point.y,
    };
    let br_point = Point {
        x: bl_point.x,
        y: tr_point.y,
    };

    return vec![tr_point, br_point, bl_point, tl_point];
}

fn winding_number(point: &Point, shape: &Vec<Point>) -> i32 {
    const RIGHT: Point = Point { x: -1, y: 1 };
    let ray = Ray {
        points: TwoPoint {
            first: *point,
            second: point + &RIGHT,
        },
    };

    let mut winds: i32 = 0;
    for index in 0..shape.len() {
        let next_index = (index + 1) % shape.len();

        let segment = ClopenSegment {
            points: TwoPoint {
                first: shape[index],
                second: shape[next_index],
            },
        };

        if intersects(&ray, &segment) {
            winds += 1;
        }
    }
    winds
}

fn outside_shape_inc(point: &Point, shape: &Vec<Point>) -> bool {
    for index in 0..shape.len() {
        let next_index = (index + 1) % shape.len();

        if is_between_incl(point, &shape[index], &shape[next_index]) {
            // println!("Point {:?} on boundary", point);
            return true;
        }
    }

    let winds = winding_number(point, shape);
    // println!("Winding number of {:?} is {}", point, winds);
    winds % 2 == 0
}

fn inside_shape_inc(point: &Point, shape: &Vec<Point>) -> bool {
    for index in 0..shape.len() {
        let next_index = (index + 1) % shape.len();

        if is_between_incl(point, &shape[index], &shape[next_index]) {
            // println!("Point {:?} on boundary", point);
            return true;
        }
    }

    let winds = winding_number(point, shape);
    // println!("Winding number of {:?} is {}", point, winds);
    winds % 2 == 1
}

fn max_coord(point: &Point) -> i64 {
    cmp::max(point.x, point.y)
}

fn max_shape_extent(shape: &Vec<Point>) -> i64 {
    let mut max_extent = -1;
    for point in shape.iter() {
        max_extent = cmp::max(max_extent, max_coord(point));
    }
    max_extent
}

fn publish_shape(shape: &Vec<Point>, rect: &Vec<Point>) {
    let file = File::create("drawing.svg").unwrap();
    let mut writer = BufWriter::new(file);

    let extent = {
        let shp_max = max_shape_extent(shape);
        let rect_max = max_shape_extent(rect);
        cmp::max(shp_max, rect_max) as f64
    };

    writeln!(
        writer,
        "<svg width=\"500\" height=\"500\" xmlns=\"http://www.w3.org/2000/svg\">"
    )
    .unwrap();

    let scale = |x: i64| (x as f64) * 500.0 / extent;

    {
        let mut points_str = String::new();
        for point in shape.iter() {
            write!(points_str, "{},{} ", scale(point.x), scale(point.y)).unwrap();
        }

        writeln!(
            writer,
            "<polygon fill=\"lightblue\" stroke=\"black\" points=\"{}\"/>",
            points_str
        )
        .unwrap();
    }

    for point in shape.iter() {
        writeln!(
            writer,
            "<circle fill=\"black\" stroke=\"none\" r=\"2\" cx=\"{}\" cy=\"{}\" />",
            scale(point.x),
            scale(point.y)
        )
        .unwrap();
    }

    {
        let mut points_str = String::new();
        for point in rect.iter() {
            write!(points_str, "{},{} ", scale(point.x), scale(point.y)).unwrap();
        }

        writeln!(
            writer,
            "<polygon fill=\"rgba(250, 100, 100, 0.5)\" points=\"{}\"/>",
            points_str
        )
        .unwrap();
    }

    writeln!(writer, "</svg>").unwrap();
}

fn main() {
    let mut max_area: i64 = -1;
    let points: Vec<Point> = parse_input();
    let shape = simplify_shape(&points);
    let mut best_point1 = Point { x: 0, y: 0 };
    let mut best_point2 = Point { x: 0, y: 0 };

    println!("Simplified shape: {:?}", shape);

    for first_point in &points {
        for second_point in &points {
            if first_point == second_point {
                continue;
            }
            let area = ((second_point.y - first_point.y).abs() + 1)
                * ((second_point.x - first_point.x).abs() + 1);

            if area <= max_area {
                continue;
            }
            println!(
                "Try {:?} {:?} with area {}",
                first_point, second_point, area
            );
            let rect = make_rect(first_point, second_point);
            // println!("Rect is {:?}", rect);

            let mut in_shape = true;

            for point in shape.iter() {
                if !outside_shape_inc(point, &rect) {
                    in_shape = false;
                    break;
                }
            }

            if in_shape {
                for point in rect.iter() {
                    if !inside_shape_inc(point, &shape) {
                        in_shape = false;
                        break;
                    }
                }
            }

            if in_shape {
                for (shp_index, shp_point) in shape.iter().enumerate() {
                    let next_shp_index = (shp_index + 1) % shape.len();
                    let next_shp_point = &shape[next_shp_index];

                    let shp_edge = OpenSegment {
                        points: TwoPoint {
                            first: *shp_point,
                            second: *next_shp_point,
                        },
                    };

                    for (rect_index, rect_point) in rect.iter().enumerate() {
                        let next_rect_index = (rect_index + 1) % rect.len();
                        let next_rect_point = &rect[next_rect_index];

                        let rect_edge = OpenSegment {
                            points: TwoPoint {
                                first: *rect_point,
                                second: *next_rect_point,
                            },
                        };

                        if crosses(&rect_edge, &shp_edge) {
                            println!(
                                "Rect edge {:?} crosses {:?}",
                                rect_edge.points, shp_edge.points
                            );
                            in_shape = false;
                            break;
                        }
                    }
                }
            }

            if in_shape {
                println!("In shape, updating");
                max_area = area;
                best_point1 = *first_point;
                best_point2 = *second_point;
            }
        }
    }

    println!("Max area {}", max_area);
    println!("Points {:?} {:?}", best_point1, best_point2);

    publish_shape(&shape, &make_rect(&best_point1, &best_point2));
}

// Too low: 159180048
// Wrong: 1560863364
// Too high: 3449890980
//
// Too high: 4749929916
