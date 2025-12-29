use std::cmp;
use std::cmp::Ordering;

use crate::point::Point;
use crate::two_point::{Crosses, OpenSegment, Ray, TwoPoint, is_between_incl};

pub fn make_rect(bound1: &Point, bound2: &Point) -> Vec<Point> {
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Orientation {
    Parallel,
    Clockwise,
    CounterClockwise,
}

fn ccw(vertex: &Point, start: &Point, end: &Point) -> Orientation {
    let diff1 = start - vertex;
    let diff2 = end - vertex;

    let left = diff1.x * diff2.y;
    let right = diff1.y * diff2.x;

    match left.cmp(&right) {
        Ordering::Less => Orientation::CounterClockwise,
        Ordering::Greater => Orientation::Clockwise,
        Ordering::Equal => Orientation::Parallel,
    }
}

fn winding_number(point: &Point, shape: &Vec<Point>) -> i64 {
    const RIGHT: Point = Point { x: 0, y: 1 };
    let ray = Ray {
        points: TwoPoint {
            first: *point,
            second: point + &RIGHT,
        },
    };
    // println!("Windings around {:?}", ray.points);

    let mut winds: i64 = 0;

    let mut first_index: Option<usize> = None;
    let mut first_orientation: Option<Orientation> = None;
    let mut first_point: Option<&Point> = None;

    for (index, point) in shape.iter().enumerate() {
        let orientation = ccw(&ray.points.first, &ray.points.second, point);
        if orientation != Orientation::Parallel {
            first_index = Some(index);
            first_orientation = Some(orientation);
            first_point = Some(point);
            break;
        }
    }

    let first_index = first_index.expect("Should be nondegenerate");
    let mut last_orientation = first_orientation.expect("Should be nondegenerate");
    let mut last_point = first_point.expect("Should be nondegenerate");

    // println!(
    //     "Found oriented {:?} {:?} {:?}",
    //     last_index, last_point, last_orientation
    // );

    for offset in 1..shape.len() {
        let next_index = (first_index + offset) % shape.len();
        let next_point = &shape[next_index];
        let next_orientation = ccw(&ray.points.first, &ray.points.second, next_point);

        if next_orientation == Orientation::Parallel {
            continue;
        }

        if next_orientation != last_orientation {
            let segment = OpenSegment {
                points: TwoPoint {
                    first: *last_point,
                    second: *next_point,
                },
            };

            if (&ray, &segment).crosses() {
                winds += 1;
            }
        }

        last_point = next_point;
        last_orientation = next_orientation;
    }

    winds
}

pub fn simplify_shape(shape: &Vec<Point>) -> Vec<Point> {
    let mut changed = true;
    let mut new_shape = shape.clone();
    while changed && new_shape.len() > 2 {
        changed = false;
        for point_index in 0..new_shape.len() {
            let prev_index = (point_index + new_shape.len() - 1) % new_shape.len();
            let next_index = (point_index + 1) % new_shape.len();

            if is_between_incl(
                &new_shape[point_index],
                &new_shape[prev_index],
                &new_shape[next_index],
            ) {
                new_shape.remove(point_index);
                changed = true;
                break;
            }
        }
    }
    new_shape
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

pub fn convex_in_concave_incl(convex: &Vec<Point>, concave: &Vec<Point>) -> bool {
    for point in convex.iter() {
        if !inside_shape_inc(point, &concave) {
            return false;
        }
    }

    for (conc_index, conc_point) in concave.iter().enumerate() {
        let next_conc_index = (conc_index + 1) % concave.len();
        let next_conc_point = &concave[next_conc_index];

        let shp_edge = OpenSegment {
            points: TwoPoint {
                first: *conc_point,
                second: *next_conc_point,
            },
        };

        for (conv_index, conv_point) in convex.iter().enumerate() {
            let next_conv_index = (conv_index + 1) % convex.len();
            let next_conv_point = &convex[next_conv_index];

            let rect_edge = OpenSegment {
                points: TwoPoint {
                    first: *conv_point,
                    second: *next_conv_point,
                },
            };

            if (&rect_edge, &shp_edge).crosses() {
                return false;
            }
        }
    }
    return true;
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test_simplify() {
        let in_shape = vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: -1 },
        ];

        let result_shape = simplify_shape(&in_shape);
        // println!("result shape, {:?}", result_shape);

        assert_eq!(
            result_shape,
            vec![Point { x: 0, y: 1 }, Point { x: 0, y: -1 }]
        )
    }
}
