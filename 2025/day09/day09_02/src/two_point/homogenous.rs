#[derive(Debug)]
pub struct HomogenousRatio {
    pub numerator: i64,
    pub denominator: i64,
}

impl HomogenousRatio {
    pub fn new(num: i64, den: i64) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct Homogenous2DPoint {
    pub x: i64,
    pub y: i64,
    pub denom: i64,
}

impl Homogenous2DPoint {
    pub fn x_ratio(&self) -> HomogenousRatio {
        HomogenousRatio::new(self.x, self.denom)
    }

    pub fn y_ratio(&self) -> HomogenousRatio {
        HomogenousRatio::new(self.y, self.denom)
    }

    pub fn normalize(&self) -> Self {
        if self.denom >= 0 {
            *self
        } else {
            Self {
                x: -self.x,
                y: -self.y,
                denom: -self.denom,
            }
        }
    }
}
