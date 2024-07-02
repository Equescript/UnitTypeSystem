mod warpper;
mod field_function;
mod float_point;
mod gradient;
mod group;
mod matrix;
mod pid;
mod vector;

use std::ops::Add;

pub use float_point::Float;
pub use vector::{Vec3, Basis, Origin, Direction, Position, Normalized};
pub use matrix::{Mat3, Mat4};

// convex


pub fn intersection<B, O>(o: &Vec3<Position<B, O>>, d: &Vec3<Direction<B, Normalized>>,
    origin: &Vec3<Position<B, O>>, normal: &Vec3<Direction<B, Normalized>>) -> Vec3<Position<B, O>>
    where B: Basis, O: Origin,
{
    let factor = (origin - o).dot(normal) / d.dot(&normal);
    o + d * factor
}

// (0, 0), (0.5, 1), (1, 0)
pub fn parabola(x: Float) -> Float {
    Float::from(4.0) * x * (Float::from(1.0) - x)
}

#[derive(Clone, Copy)]
pub struct Phase {
    value: Float,
}

impl Phase {
    pub const ZERO: Self = Self { value: float_point::ZERO };
    pub const QUARTER: Self = Self { value: float_point::QUARTER };
    pub const HALF: Self = Self { value: float_point::HALF };
    pub const THREE_QUARTERS: Self = Self { value: float_point::THREE_QUARTERS };
    pub fn new(value: Float) -> Self {
        Self { value }
    }
    pub fn value(&self) -> Float {
        self.value
    }
}

impl std::ops::Neg for Phase {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { value: Float::from(1.0) - self.value }
    }
}

impl std::ops::Add for Phase {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { value: (self.value + rhs.value).fract() }
    }
}

impl std::ops::Sub for Phase {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}

impl std::ops::Mul<Float> for Phase {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self::Output {
        Self { value: self.value * rhs }
    }
}