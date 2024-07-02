use std::ops::{Add, Sub, Mul, Div};
use super::super::{Float, Mat3};
use super::Matrix3;

type glm_Vec3 = nalgebra_glm::DVec3;

#[derive(Clone, Copy)]
pub struct Vector3 {
    v: glm_Vec3,
}

impl From<glm_Vec3> for Vector3 {
    fn from(value: glm_Vec3) -> Self {
        Self { v: value }
    }
}

impl Vector3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        glm_Vec3::new(x.into(), y.into(), z.into()).into()
    }
    pub fn zeros() -> Self {
        glm_Vec3::zeros().into()
    }
    pub fn x(&self) -> Float {
        self.v.x.into()
    }
    pub fn y(&self) -> Float {
        self.v.y.into()
    }
    pub fn z(&self) -> Float {
        self.v.z.into()
    }
    pub fn magnitude(&self) -> Float {
        self.v.magnitude().into()
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self { v: self.v.cross(&other.v) }
    }
    pub fn dot(&self, other: &Self) -> Float {
        self.v.dot(&other.v).into()
    }
    pub fn normalize(&self) -> Self {
        self.v.normalize().into()
    }
    pub fn to_rotation_mat3(&self) -> Matrix3 {
        nalgebra_glm::mat4_to_mat3(&nalgebra_glm::rotation(self.v.magnitude(), &self.v)).into()
    }
    /* pub fn to_rotation_mat4(&self) -> Mat4 {
        nalgebra_glm::rotation(self.v.magnitude(), &self.v).into()
    } */
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

impl<'a > Add<&'a Vector3> for &'a Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

impl Mul<Float> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Float) -> Self::Output {
        (self.v * rhs.v).into()
    }
}

impl Mul<Float> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Float) -> Self::Output {
        (self.v * rhs.v).into()
    }
}

impl Mul<&Float> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Float) -> Self::Output {
        (self.v * rhs.v).into()
    }
}

impl Mul<&Float> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Float) -> Self::Output {
        (self.v * rhs.v).into()
    }
}
