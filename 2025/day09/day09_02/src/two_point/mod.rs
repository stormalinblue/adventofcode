mod homogenous;

use crate::point::Point;
use homogenous::Homogenous2DPoint;

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

pub fn intersects(ray: &Ray, point: &Point) -> bool {
    let diff1 = point - &ray.points.first;
    let diff2 = &ray.points.second - &ray.points.first;

    if diff1.x * diff2.y != diff1.y * diff2.x {
        // Points should be collinear
        false
    } else {
        // Point should be in the same direction
        diff1.x * diff2.x + diff1.y * diff2.y >= 0
    }
}

pub trait Crosses {
    fn crosses(&self) -> bool;
}

impl Crosses for (&OpenSegment, &OpenSegment) {
    fn crosses(&self) -> bool {
        let (segment1, segment2) = *self;
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
}

impl Crosses for (&Ray, &OpenSegment) {
    fn crosses(&self) -> bool {
        let (ray, segment) = *self;
        let times = intersection_time(&ray.points, &segment.points).normalize();
        // println!("Times {:?}", times);
        if times.denom == 0 {
            false
        } else {
            let ray_time = times.x_ratio();
            let segment_time = times.y_ratio();
            // we must have 0 < segment1_time < 1
            // we must have 0 < segment2_time < 1

            0 <= ray_time.numerator
                && 0 < segment_time.numerator
                && segment_time.numerator < segment_time.denominator
        }
    }
}

#[cfg(test)]
pub mod test {
    use homogenous::HomogenousRatio;

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
    fn test_crosses_ray_open() {
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

        let segment = OpenSegment {
            points: TwoPoint {
                first: seg_start,
                second: seg_end,
            },
        };

        assert!((&left_ray, &segment).crosses());
        assert!(!(&right_ray, &segment).crosses());
        assert!((&middle_ray, &segment).crosses());
        assert!((&left_point_ray, &segment).crosses());
        assert!(!(&right_point_ray, &segment).crosses());
        assert!((&seg_start_ray, &segment).crosses());
        assert!((&par_ray_inner, &segment).crosses());
        assert!((&par_ray_outer, &segment).crosses());
    }
}
