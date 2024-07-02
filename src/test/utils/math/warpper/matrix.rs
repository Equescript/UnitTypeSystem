use std::ops::{Add, Sub, Mul, Div};
use super::super::{Float, Mat3};

type glm_Mat3 = nalgebra_glm::DMat3;
type glm_Mat4 = nalgebra_glm::DMat4;

#[derive(Clone, Copy)]
pub struct Matrix3 {
    m: glm_Mat3,
}

impl From<glm_Mat3> for Matrix3 {
    fn from(value: glm_Mat3) -> Self {
        Self { m: value }
    }
}

impl Matrix3 {
    pub fn identity() -> Self {
        Self { m: glm_Mat3::identity() }
    }
    pub fn inverse(&self) -> Self {
        nalgebra_glm::inverse(&self.m).into()
    }
    pub fn try_inverse(&self) -> Option<Self> {
        match self.m.try_inverse() {
            Some(m) => Some(m.into()),
            None => None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Matrix4 {
    m: glm_Mat4
}