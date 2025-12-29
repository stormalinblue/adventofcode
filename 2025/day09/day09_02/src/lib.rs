pub mod point;

use point::Point;

#[derive(Debug)]
pub struct TwoPoint {
    pub first: Point,
    pub second: Point,
}

impl TwoPoint {
    fn diff(&self) -> Point {
        return &self.second - &self.first;
    }
}

pub struct ClopenSegment {
    pub points: TwoPoint,
}

pub struct OpenSegment {
    pub points: TwoPoint,
}

pub struct Ray {
    pub points: TwoPoint,
}

impl Ray {
    pub fn from_point_and_dir(point: &Point, dir: &Point) -> Self {
        Self {
            points: TwoPoint {
                first: *point,
                second: point + dir,
            },
        }
    }
}

#[derive(Debug)]
pub struct HomogenousRatio {
    numerator: i64,
    denominator: i64,
}

impl HomogenousRatio {
    fn new(num: i64, den: i64) -> Self {
        Self {
            numerator: num,
            denominator: den,
        }
    }
}

impl PartialEq for HomogenousRatio {
    fn eq(&self, other: &Self) -> bool {
        other.denominator * self.numerator == other.numerator * self.denominator
    }
}
impl Eq for HomogenousRatio {}

#[derive(Debug)]
pub struct Homogenous2DPoint {
    x: i64,
    y: i64,
    denom: i64,
}

impl Homogenous2DPoint {
    fn x_ratio(&self) -> HomogenousRatio {
        HomogenousRatio::new(self.x, self.denom)
    }

    fn y_ratio(&self) -> HomogenousRatio {
        HomogenousRatio::new(self.y, self.denom)
    }

    fn normalize(&self) -> Self {
        if self.denom >= 0 {
            Self {
                x: self.x,
                y: self.y,
                denom: self.denom,
            }
        } else {
            Self {
                x: -self.x,
                y: -self.y,
                denom: -self.denom,
            }
        }
    }
}

pub fn intersection_time(a: &TwoPoint, b: &TwoPoint) -> Homogenous2DPoint {
    let delta_a = a.diff();
    let delta_b = b.diff();

    let delta_both = &a.first - &b.first;

    // println!("Delta a {:?}", delta_a);
    // println!("Delta b {:?}", delta_b);
    // println!("Delta both {:?}", delta_both);

    Homogenous2DPoint {
        x: delta_both.x * delta_b.y - delta_both.y * delta_b.x,
        y: delta_both.x * delta_a.y - delta_both.y * delta_a.x,
        denom: delta_a.y * delta_b.x - delta_a.x * delta_b.y,
    }
}

pub fn is_between_incl(a: &Point, start: &Point, end: &Point) -> bool {
    let diff1 = a - start;
    let diff2 = a - end;

    // println!("Points are {:?} {:?} {:?}", a, start, end);
    // println!("Diffs are {:?} {:?}", diff1, diff2);
    if diff1.x * diff2.y != diff1.y * diff2.x {
        // Points should be collinear
        false
    } else {
        (diff1.x * diff2.x + diff1.y * diff2.y) <= 0
    }
}

pub fn intersects(ray: &Ray, segment: &ClopenSegment) -> bool {
    let times = intersection_time(&ray.points, &segment.points).normalize();
    // println!("Times {:?}", times);
    if times.denom == 0 {
        times.x == 0 && times.y == 0
    } else {
        let segment_time = times.y_ratio();
        let ray_time = times.x_ratio();
        // we must have 0 <= segment_time < 1
        // we must have 0 <= ray_time

        0 <= segment_time.numerator
            && segment_time.numerator < segment_time.denominator
            && 0 <= ray_time.numerator
    }
}

pub fn crosses(segment1: &OpenSegment, segment2: &OpenSegment) -> bool {
    let times = intersection_time(&segment1.points, &segment2.points).normalize();
    // println!("Times {:?}", times);
    if times.denom == 0 {
        false
    } else {
        let segment1_time = times.x_ratio();
        let segment2_time = times.y_ratio();
        // we must have 0 < segment1_time < 1
        // we must have 0 < segment2_time < 1

        0 < segment1_time.numerator
            && segment1_time.numerator < segment1_time.denominator
            && 0 < segment2_time.numerator
            && segment2_time.numerator < segment2_time.denominator
    }
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

#[cfg(test)]
pub mod test {
    use super::*;

    use super::point::Point;

    #[test]
    fn test_times() {
        let right = Point { x: 1, y: 0 };
        let down = Point { x: 0, y: -1 };

        for x in -5..=5 {
            for y in 1..=10 {
                let x_point = Point { x, y: 0 };
                let y_point = Point { x: 0, y };

                let x_ray = TwoPoint {
                    first: x_point,
                    second: &x_point + &right,
                };
                let y_ray = TwoPoint {
                    first: y_point,
                    second: &y_point + &down,
                };

                let itime = intersection_time(&x_ray, &y_ray);

                assert_eq!(
                    itime.x_ratio(),
                    HomogenousRatio::new(-x, 1),
                    "x time mismatch"
                );
                assert_eq!(
                    itime.y_ratio(),
                    HomogenousRatio::new(y, 1),
                    "y time mismatch"
                );
            }
        }
    }

    #[test]
    fn test_intersects_ray_clopen() {
        let up_dir = Point { x: 0, y: 1 };

        let left = Point { x: -4, y: 0 };
        let right = Point { x: 4, y: 0 };
        let middle = Point { x: 0, y: 0 };

        let left_point_on_x = Point { x: -2, y: 0 };
        let right_point_on_x = Point { x: 2, y: 0 };

        let seg_start = Point { x: -2, y: 5 };
        let seg_end = Point { x: 2, y: 5 };

        let outer_point_left = Point { x: -3, y: 5 };
        let outer_point_right = Point { x: 3, y: 5 };

        let inner_point_left = Point { x: -3, y: 5 };
        let inner_point_right = Point { x: 3, y: 5 };

        let left_ray = Ray {
            points: TwoPoint {
                first: left,
                second: &left + &up_dir,
            },
        };
        let middle_ray = Ray {
            points: TwoPoint {
                first: middle,
                second: &middle + &up_dir,
            },
        };
        let right_ray = Ray {
            points: TwoPoint {
                first: right,
                second: &right + &up_dir,
            },
        };
        let left_point_ray = Ray {
            points: TwoPoint {
                first: left_point_on_x,
                second: &left_point_on_x + &up_dir,
            },
        };
        let right_point_ray = Ray {
            points: TwoPoint {
                first: right_point_on_x,
                second: &right_point_on_x + &up_dir,
            },
        };

        let seg_start_ray = Ray {
            points: TwoPoint {
                first: seg_start,
                second: &seg_start + &up_dir,
            },
        };

        let par_ray_outer = Ray {
            points: TwoPoint {
                first: outer_point_left,
                second: outer_point_right,
            },
        };

        let par_ray_inner = Ray {
            points: TwoPoint {
                first: inner_point_left,
                second: inner_point_right,
            },
        };

        let segment = ClopenSegment {
            points: TwoPoint {
                first: seg_start,
                second: seg_end,
            },
        };

        assert!(!intersects(&left_ray, &segment));
        assert!(!intersects(&right_ray, &segment));
        assert!(intersects(&middle_ray, &segment));
        assert!(intersects(&left_point_ray, &segment));
        assert!(!intersects(&right_point_ray, &segment));
        assert!(intersects(&seg_start_ray, &segment));
        assert!(intersects(&par_ray_inner, &segment));
        assert!(intersects(&par_ray_outer, &segment));
    }

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
