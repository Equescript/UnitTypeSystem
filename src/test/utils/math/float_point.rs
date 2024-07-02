use std::ops::{Add, Sub, Mul, Div};
use super::super::physics::UnitlessValue;

pub const ZERO: Float = Float { v: 0.0 };
pub const QUARTER: Float = Float { v: 0.25 };
pub const HALF: Float = Float { v: 0.5 };
pub const THREE_QUARTERS: Float = Float { v: 0.75 };

#[derive(Clone, Copy)]
pub struct Float {
    pub v: f64,
}

impl UnitlessValue for Float {}

impl From<f64> for Float {
    fn from(value: f64) -> Self {
        Self { v: value }
    }
}

impl From<Float> for f64 {
    fn from(value: Float) -> Self {
        value.v
    }
}

impl Float {
    pub fn fract(self) -> Float {
        self.v.fract().into()
    }
}

impl Add for Float {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

impl Sub for Float {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.v - rhs.v).into()
    }
}

impl Mul for Float {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.v * rhs.v).into()
    }
}

impl Div for Float {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        (self.v / rhs.v).into()
    }
}
