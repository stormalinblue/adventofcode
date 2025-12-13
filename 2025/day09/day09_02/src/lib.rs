use std::cmp;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl<'b> Sub<&'b Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &'b Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'b> Sub<&'b Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &'b Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'b> Add<&'b Point> for &Point {
    type Output = Point;

    fn add(self, rhs: &'b Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn in_closed_interval(interval_start: i64, interval_end: i64, test: i64) -> bool {
    ((interval_start <= test) && (test <= interval_end))
        || ((interval_start >= test) && (test >= interval_end))
}

fn in_clopen_interval(interval_start: i64, interval_end: i64, test: i64) -> bool {
    ((interval_start <= test) && (test < interval_end))
        || ((interval_start >= test) && (test > interval_end))
}

#[derive(Debug)]
struct TwoPointSegment {
    first: Point,
    second: Point,
}

#[derive(Debug)]
struct HomogenousSlope {
    run: i64,
    rise: i64,
}

impl TwoPointSegment {
    pub fn slope(&self) -> HomogenousSlope {
        let diff = &self.second - &self.first;

        HomogenousSlope {
            run: diff.x,
            rise: diff.y,
        }
    }

    pub fn is_parallel(&self, other: &TwoPointSegment) -> bool {
        let my_slope = self.slope();
        let other_slope = other.slope();

        (other_slope.rise * my_slope.run - other_slope.run * my_slope.rise) == 0
    }

    pub fn has_point_inc(&self, point: &Point) -> bool {
        if in_closed_interval(self.first.x, self.second.x, point.x)
            && in_closed_interval(self.first.y, self.second.y, point.y)
        {
            self.is_parallel(&TwoPointSegment {
                first: self.first,
                second: *point,
            })
        } else {
            false
        }
    }

    pub fn has_point_clopen(&self, point: &Point) -> bool {
        if in_clopen_interval(self.first.x, self.second.x, point.x)
            && in_clopen_interval(self.first.y, self.second.y, point.y)
        {
            self.is_parallel(&TwoPointSegment {
                first: self.first,
                second: *point,
            })
        } else {
            false
        }
    }
}

struct Ray {
    start: Point,
    slope: HomogenousSlope,
}
