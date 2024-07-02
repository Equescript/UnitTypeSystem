use std::{marker::PhantomData, ops::{Add, Sub, Mul, Div}};
use super::{Vec3, Direction};
use super::super::physics::UnitlessValue;
use super::vector::{Basis as VecSpaceBasis, Rotation as VecRotation};

use super::warpper::{Matrix3, Matrix4};

#[derive(Clone, Copy)]
pub struct Mat3<Prop> where Prop: MatProperty {
    m: Matrix3,
    prop: PhantomData<Prop>,
}

impl<Prop> UnitlessValue for Mat3<Prop> where Prop: MatProperty {}

impl<Prop> From<Matrix3> for Mat3<Prop> where Prop: MatProperty {
    fn from(value: Matrix3) -> Self {
        Self { m: value, prop: PhantomData }
    }
}

impl<Prop> Mat3<Prop> where Prop: MatProperty {
    pub fn identity() -> Self {
        Matrix3::identity().into()
    }
    pub fn value(&self) -> Matrix3 {
        self.m
    }
}

impl<B> From<Vec3<VecRotation<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    fn from(value: Vec3<VecRotation<B>>) -> Self {
        value.to_mat()
    }
}

impl<B> Mat3<Rotation<B>> where B: VecSpaceBasis {
    /* pub fn to_vec(&self) -> Vec3<VecRotation<B>> {
        todo!()
    } */
    pub fn inverse(&self) -> Self {
        self.m.inverse().into()
    }
}

impl<B> Mat3<Basis<B>> where B: VecSpaceBasis {
    pub fn inverse(&self) -> Mat3<BasisInverse<B>> {
        self.m.inverse().into()
    }
}

impl<B> Mat3<BasisInverse<B>> where B: VecSpaceBasis {
    pub fn inverse(&self) -> Mat3<Basis<B>> {
        self.m.inverse().into()
    }
}

// Rotation * Rotation = Rotation
impl<B> Mul<Mat3<Rotation<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: Mat3<Rotation<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<Mat3<Rotation<B>>> for &Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: Mat3<Rotation<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Mat3<Rotation<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: &Mat3<Rotation<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Mat3<Rotation<B>>> for &Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: &Mat3<Rotation<B>>) -> Self::Output {
        todo!()
    }
}

// Rotation * Basis = Basis
impl<B> Mul<Mat3<Basis<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Basis<B>>;
    fn mul(self, rhs: Mat3<Basis<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<Mat3<Basis<B>>> for &Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Basis<B>>;
    fn mul(self, rhs: Mat3<Basis<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Mat3<Basis<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Basis<B>>;
    fn mul(self, rhs: &Mat3<Basis<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Mat3<Basis<B>>> for &Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Mat3<Basis<B>>;
    fn mul(self, rhs: &Mat3<Basis<B>>) -> Self::Output {
        todo!()
    }
}

// Rotation = Basis * BasisInverse
impl<B> Mul<Mat3<BasisInverse<B>>> for Mat3<Basis<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: Mat3<BasisInverse<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<Mat3<BasisInverse<B>>> for &Mat3<Basis<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: Mat3<BasisInverse<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Mat3<BasisInverse<B>>> for Mat3<Basis<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: &Mat3<BasisInverse<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Mat3<BasisInverse<B>>> for &Mat3<Basis<B>> where B: VecSpaceBasis {
    type Output = Mat3<Rotation<B>>;
    fn mul(self, rhs: &Mat3<BasisInverse<B>>) -> Self::Output {
        todo!()
    }
}

// Rotation * DirectionVec3 = DirectionVec3
impl<B> Mul<Vec3<Direction<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: Vec3<Direction<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<Vec3<Direction<B>>> for &Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: Vec3<Direction<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Vec3<Direction<B>>> for Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: &Vec3<Direction<B>>) -> Self::Output {
        todo!()
    }
}
impl<B> Mul<&Vec3<Direction<B>>> for &Mat3<Rotation<B>> where B: VecSpaceBasis {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: &Vec3<Direction<B>>) -> Self::Output {
        todo!()
    }
}

#[derive(Clone, Copy)]
pub struct Mat4 {
    m: Matrix4
}

pub trait MatProperty {}

#[derive(Clone, Copy)]
pub struct Rotation<B> where B: VecSpaceBasis {
    basis: PhantomData<B>,
}
impl<B> MatProperty for Rotation<B> where B: VecSpaceBasis {}

#[derive(Clone, Copy)]
pub struct Basis<B> where B: VecSpaceBasis {
    basis: PhantomData<B>,
}
impl<B> MatProperty for Basis<B> where B: VecSpaceBasis {}

#[derive(Clone, Copy)]
pub struct BasisInverse<B> where B: VecSpaceBasis {
    basis: PhantomData<B>,
}
impl<B> MatProperty for BasisInverse<B> where B: VecSpaceBasis {}


/* pub struct BasisMat3 {
    m: Mat3
}

pub struct RotationMat3 {
    m: Mat3
} */

fn test() {
    nalgebra_glm::Mat3::zeros() * nalgebra_glm::Mat3::zeros();
}
