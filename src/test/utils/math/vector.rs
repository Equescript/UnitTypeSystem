use std::{marker::PhantomData, ops::{Add, Sub, Mul, Div}};

use super::super::property::Singleton;
use super::super::physics::UnitlessValue;

use super::warpper::Vector3;
use super::{float_point::Float, Mat3};
use super::matrix::Rotation as MatRotation;



#[derive(Clone, Copy)]
pub struct Vec3<Prop> where Prop: VecProperty {
    v: Vector3,
    prop: PhantomData<Prop>
}

impl<Prop> UnitlessValue for Vec3<Prop> where Prop: VecProperty {}

impl<Prop> From<Vector3> for Vec3<Prop> where Prop: VecProperty {
    fn from(value: Vector3) -> Self {
        Self { v: value, prop: PhantomData }
    }
}

impl<Prop> Vec3<Prop> where Prop: VecProperty {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Vector3::new(x, y, z).into()
    }
    pub fn zeros() -> Self {
        Vector3::zeros().into()
    }
    pub fn x(&self) -> Float { self.v.x() }
    pub fn y(&self) -> Float { self.v.y() }
    pub fn z(&self) -> Float { self.v.z() }
    pub fn magnitude(&self) -> Float {
        self.v.magnitude()
    }
    pub fn value(&self) -> Vector3 {
        self.v
    }
}

impl<B, O, N> From<Vec3<Direction<B, N>>> for Vec3<Position<B, O>> where B: Basis, O: Origin, N: VecNormalizedProperty {
    fn from(value: Vec3<Direction<B, N>>) -> Self {
        value.to_position()
    }
}

impl<B, N> Vec3<Direction<B, N>> where B: Basis, N: VecNormalizedProperty {
    pub fn cross(&self, other: &Vec3<Direction<B, N>>) -> Self {
        self.v.cross(&other.v).into()
    }
}

impl<B, N> Vec3<Direction<B, N>> where B: Basis, N: VecNormalizedProperty {
    pub fn to_position<O: Origin>(&self) -> Vec3<Position<B, O>> {
        self.v.into()
    }
    pub fn dot<N2>(&self, other: &Vec3<Direction<B, N2>>) -> Float where N2: VecNormalizedProperty {
        self.v.dot(&other.v)
    }
    pub fn normalize(&self) -> Vec3<Direction<B, Normalized>> {
        self.v.normalize().into()
    }
}

impl<B, O> From<Vec3<Position<B, O>>> for Vec3<Direction<B>> where B: Basis, O: Origin {
    fn from(value: Vec3<Position<B, O>>) -> Self {
        value.to_direction()
    }
}

impl<B, O> Vec3<Position<B, O>> where B: Basis, O: Origin {
    pub fn to_direction(&self) -> Vec3<Direction<B>> {
        Vec3 { v: self.v, prop: PhantomData }
    }
}

impl<B> Vec3<Rotation<B>> where B: Basis {
    pub fn to_mat(&self) -> Mat3<MatRotation<B>> {
        self.v.to_rotation_mat3().into()
    }
}

// Direction + Direction = Direction
impl<B> Add<Vec3<Direction<B>>> for Vec3<Direction<B>> where B: Basis {
    type Output = Vec3<Direction<B>>;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B> Add<Vec3<Direction<B>>> for &Vec3<Direction<B>> where B: Basis {
    type Output = Vec3<Direction<B>>;
    fn add(self, rhs: Vec3<Direction<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B> Add<&Vec3<Direction<B>>> for Vec3<Direction<B>> where B: Basis {
    type Output = Vec3<Direction<B>>;
    fn add(self, rhs: &Vec3<Direction<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<'a, B> Add<&'a Vec3<Direction<B>>> for &'a Vec3<Direction<B>> where B: Basis {
    type Output = Vec3<Direction<B>>;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

/* // Position + Position
impl<B, O> Add<Vec3<Position<B, O>>> for Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<'a, B, O> Add<&'a Vec3<Position<B, O>>> for &'a Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
} */

// Position - Position = Direction
impl<B, O> Sub<Vec3<Position<B, O>>> for Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Direction<B>>;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Sub<Vec3<Position<B, O>>> for &Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Direction<B>>;
    fn sub(self, rhs: Vec3<Position<B, O>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Sub<&Vec3<Position<B, O>>> for Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Direction<B>>;
    fn sub(self, rhs: &Vec3<Position<B, O>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<'a, B, O> Sub<&'a Vec3<Position<B, O>>> for &'a Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Direction<B>>;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

// Rotation + Rotation = Rotation
impl<B> Add<Vec3<Rotation<B>>> for Vec3<Rotation<B>> where B: Basis{
    type Output = Vec3<Rotation<B>>;
    fn add(self, rhs: Vec3<Rotation<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B> Add<Vec3<Rotation<B>>> for &Vec3<Rotation<B>> where B: Basis{
    type Output = Vec3<Rotation<B>>;
    fn add(self, rhs: Vec3<Rotation<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B> Add<&Vec3<Rotation<B>>> for Vec3<Rotation<B>> where B: Basis{
    type Output = Vec3<Rotation<B>>;
    fn add(self, rhs: &Vec3<Rotation<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B> Add<&Vec3<Rotation<B>>> for &Vec3<Rotation<B>> where B: Basis{
    type Output = Vec3<Rotation<B>>;
    fn add(self, rhs: &Vec3<Rotation<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

// Position + Direction = Position
impl<B, O> Add<Vec3<Direction<B>>> for Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: Vec3<Direction<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Add<Vec3<Direction<B>>> for &Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: Vec3<Direction<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Add<&Vec3<Direction<B>>> for Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: &Vec3<Direction<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Add<&Vec3<Direction<B>>> for &Vec3<Position<B, O>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: &Vec3<Direction<B>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

// Direction + Position = Position
impl<B, O> Add<Vec3<Position<B, O>>> for Vec3<Direction<B>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: Vec3<Position<B, O>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Add<Vec3<Position<B, O>>> for &Vec3<Direction<B>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: Vec3<Position<B, O>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Add<&Vec3<Position<B, O>>> for Vec3<Direction<B>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: &Vec3<Position<B, O>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}
impl<B, O> Add<&Vec3<Position<B, O>>> for &Vec3<Direction<B>> where B: Basis, O: Origin {
    type Output = Vec3<Position<B, O>>;
    fn add(self, rhs: &Vec3<Position<B, O>>) -> Self::Output {
        (self.v + rhs.v).into()
    }
}

// Direction * Float = Direction
impl<B, N> Mul<Float> for Vec3<Direction<B, N>> where B: Basis, N: VecNormalizedProperty {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: Float) -> Self::Output {
        (self.v * rhs).into()
    }
}
impl<B, N> Mul<Float> for &Vec3<Direction<B, N>> where B: Basis, N: VecNormalizedProperty {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: Float) -> Self::Output {
        (self.v * rhs).into()
    }
}
impl<B, N> Mul<&Float> for Vec3<Direction<B, N>> where B: Basis, N: VecNormalizedProperty {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: &Float) -> Self::Output {
        (self.v * rhs).into()
    }
}
impl<B, N> Mul<&Float> for &Vec3<Direction<B, N>> where B: Basis, N: VecNormalizedProperty {
    type Output = Vec3<Direction<B>>;
    fn mul(self, rhs: &Float) -> Self::Output {
        (self.v * rhs).into()
    }
}

// Rotation * Float = Rotation
impl<B> Mul<Float> for Vec3<Rotation<B>> where B: Basis{
    type Output = Vec3<Rotation<B>>;
    fn mul(self, rhs: Float) -> Self::Output {
        (self.v * rhs).into()
    }
}
impl<B> Mul<Float> for &Vec3<Rotation<B>> where B: Basis {
    type Output = Vec3<Rotation<B>>;
    fn mul(self, rhs: Float) -> Self::Output {
        (self.v * rhs).into()
    }
}
impl<B> Mul<&Float> for Vec3<Rotation<B>> where B: Basis {
    type Output = Vec3<Rotation<B>>;
    fn mul(self, rhs: &Float) -> Self::Output {
        (self.v * rhs).into()
    }
}
impl<B> Mul<&Float> for &Vec3<Rotation<B>> where B: Basis {
    type Output = Vec3<Rotation<B>>;
    fn mul(self, rhs: &Float) -> Self::Output {
        (self.v * rhs).into()
    }
}


static mut DEFAULT: () = ();
impl Singleton for () {
    fn instance() -> &'static Self {
        unsafe{ &DEFAULT }
    }
    fn instance_mut() -> &'static mut Self {
        unsafe { &mut DEFAULT }
    }
}

pub trait Basis: Singleton {
    /* fn transformation(&self) -> Mat3;
    fn transformation_inverse(&self) -> Mat3;
    fn transformation_to<B: Basis>(&self, other: B) -> Mat3 {
        // self.transformation() * v == other.transformation() * v'
        // other.transformation_inverse() * self.transformation() * v == v'
        other.transformation_inverse() * self.transformation()
    } */
}

impl Basis for () {
    /* fn transformation(&self) -> Mat3 {
        Mat3::identity()
    }
    fn transformation_inverse(&self) -> Mat3 {
        Mat3::identity()
    } */
}

pub trait Origin: Singleton {
    fn location(&self) -> Vec3<Position>;
}

impl Origin for () {
    fn location(&self) -> Vec3<Position> {
        Vec3::zeros()
    }
}

pub trait VecProperty {}

#[derive(Clone, Copy)]
pub struct Direction<B = (), N = ()> where B: Basis, N: VecNormalizedProperty {
    basis: PhantomData<B>,
    normalized: PhantomData<N>
}
impl<B, N> VecProperty for Direction<B, N> where B: Basis, N: VecNormalizedProperty {}

pub trait VecNormalizedProperty {}
impl VecNormalizedProperty for () {}

#[derive(Clone, Copy)]
pub struct Normalized;
impl VecNormalizedProperty for Normalized {}



#[derive(Clone, Copy)]
pub struct Position<B = (), O = ()> where B: Basis, O: Origin {
    basis: PhantomData<B>,
    origin: PhantomData<O>,
}
impl<B, O> VecProperty for Position<B, O> where B: Basis, O: Origin {}

pub struct Rotation<B = ()> where B: Basis {
    basis: PhantomData<B>,
}
impl<B> VecProperty for Rotation<B> where B: Basis {}
