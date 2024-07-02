use std::ops::{Add, Sub, Mul, Div};
use super::units::{Unit, Unify, UnitOperation, Second, Frame, Meter, Kg, Radian, Degree};
use super::units::dimension::DimensionType;
use super::units::base_units::{
    TimeBaseUnit,
    LengthBaseUnit,
    MassBaseUnit,
    AngleBaseUnit
};

/// 允许无单位值直接与有单位值运算
pub trait UnitlessValue {}

#[derive(Clone, Copy)]
pub struct PhysicalQuantity<T, U> (T, U);

/* impl<T, U> Clone for PhysicalQuantity<T, U> where T: Clone, U: Clone {
    fn clone(&self) -> Self {
        PhysicalQuantity(self.0.clone(), self.1.clone())
    }
}

impl<T, U> Copy for PhysicalQuantity<T, U> where T: Clone + Copy, U: Clone + Copy {} */

pub type PQ<T, U> = PhysicalQuantity<T, U>;

impl<T, U> PQ<T, U> {
    pub fn value(self) -> T {
        self.0
    }
}

impl<T, U> From<T> for PQ<T, U> where U: Default {
    fn from(value: T) -> Self {
        Self(value, U::default())
    }
}

impl<T, U1, U2> Unify for PQ<T, UnitOperation<U1, U2>>
where UnitOperation<U1, U2>: Unify
{
    type UnifyOutput = PQ<T, <UnitOperation<U1, U2> as Unify>::UnifyOutput>;
    fn unify(self) -> Self::UnifyOutput {
        PhysicalQuantity(self.0, self.1.unify())
    }
}

/* impl<T, U> PQ<T, U> {
    pub fn new(value: T, unit: U) -> Self {
        Self(value, unit)
    }
} */

impl<T, U> PQ<T, U> where U: Default {
    pub fn new(value: T) -> Self {
        Self(value, U::default())
    }
}

impl<T1, T2, U> Add<PQ<T2, U>> for PQ<T1, U>
where T1: Add<T2>
{
    type Output = PQ<<T1 as Add<T2>>::Output, U>;
    fn add(self, rhs: PQ<T2, U>) -> Self::Output {
        PhysicalQuantity(self.0 + rhs.0, self.1)
    }
}

impl<T1, T2, U> Sub<PQ<T2, U>> for PQ<T1, U>
where T1: Sub<T2>
{
    type Output = PQ<<T1 as Sub<T2>>::Output, U>;
    fn sub(self, rhs: PQ<T2, U>) -> Self::Output {
        PhysicalQuantity(self.0 - rhs.0, self.1)
    }
}

impl<T1, U1, T2, U2> Mul<PQ<T2, U2>> for PQ<T1, U1>
where T1: Mul<T2>, //UnitOperation<U1, U2>: Unify
{
    type Output = PQ<<T1 as Mul<T2>>::Output, UnitOperation<U1, U2>>;
    fn mul(self, rhs: PQ<T2, U2>) -> Self::Output {
        PhysicalQuantity(self.0 * rhs.0, UnitOperation(self.1, rhs.1))
    }
}

impl<T1, T2, U> Mul<T2> for PQ<T1, U>
where T1: Mul<T2>, T2: UnitlessValue
{
    type Output = PQ<<T1 as Mul<T2>>::Output, U>;
    fn mul(self, rhs: T2) -> Self::Output {
        PhysicalQuantity(self.0 * rhs, self.1)
    }
}

impl<T1, U1, T2, T, L, M, A> Div<PQ<T2, Unit<T, L, M, A>>> for PQ<T1, U1>
where T1: Div<T2>, //UnitOperation<U1, U2>: Unify
T: TimeBaseUnit,
L: LengthBaseUnit,
M: MassBaseUnit,
A: AngleBaseUnit,
<T as DimensionType>::Inverse: TimeBaseUnit,
<L as DimensionType>::Inverse: LengthBaseUnit,
<M as DimensionType>::Inverse: MassBaseUnit,
<A as DimensionType>::Inverse: AngleBaseUnit
{
    type Output = PQ<<T1 as Div<T2>>::Output, UnitOperation<U1, Unit<T::Inverse, L::Inverse, M::Inverse, A::Inverse>>>;
    fn div(self, rhs: PQ<T2, Unit<T, L, M, A>>) -> Self::Output {
        PhysicalQuantity(self.0 / rhs.0, UnitOperation(self.1, Unit(rhs.1.0.inverse(), rhs.1.1.inverse(), rhs.1.2.inverse(), rhs.1.3.inverse())))
    }
}

impl<T1, T2, U> Div<T2> for PQ<T1, U>
where T1: Div<T2>, T2: UnitlessValue
{
    type Output = PQ<<T1 as Div<T2>>::Output, U>;
    fn div(self, rhs: T2) -> Self::Output {
        PhysicalQuantity(self.0 / rhs, self.1)
    }
}


/* fn test() {
    // let velocity = PQ::new(3.0f64, Unit::<Second<N, I1>, Meter<P, I1>, Unitless, Unitless>::default());
    // let length = PQ::new(6.0f64, Unit::<Unitless, Meter<P, I1>, Unitless, Unitless>::default());
    // let time = PQ::new(2.0f64, Unit::<Second<P, I1>, Unitless, Unitless, Unitless>::default());
    let velocity = PQ::<f64, Unit<Second<N, I1>, Meter<P, I1>, Unitless, Unitless>>::new(3.0);
    let time = PQ::<f64, Unit<Second<P, I1>, Unitless, Unitless, Unitless>>::new(2.0);
    let length: PQ<f64, Unit<Unitless, Meter<P, I1>, Unitless, Unitless>> = (velocity * time).unify();
    // let a = UnitOperation(Unit::<Second<N, I1>, Meter<P, I1>, Unitless, Unitless>::default(), Unit::<Second<P, I1>, Unitless, Unitless, Unitless>::default()).unify();
} */
