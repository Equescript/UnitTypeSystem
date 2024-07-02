use super::dimension::{SignType, IdentityType, BaseUnit, Unitless, O, NotZeroIdentityType, TypeUnify, DimensionType};

pub trait BaseUnitType: Clone + Copy {}
pub trait TimeUnitType: BaseUnitType {}
pub trait LengthUnitType: BaseUnitType {}
pub trait MassUnitType: BaseUnitType {}
pub trait AngleUnitType: BaseUnitType {}

#[derive(Clone, Copy, Default)]
pub struct Second;
impl BaseUnitType for Second {}
impl TimeUnitType for Second {}

#[derive(Clone, Copy, Default)]
pub struct Frame;
impl BaseUnitType for Frame {}
impl TimeUnitType for Frame {}

#[derive(Clone, Copy, Default)]
pub struct Meter;
impl BaseUnitType for Meter {}
impl LengthUnitType for Meter {}

#[derive(Clone, Copy, Default)]
pub struct Kg;
impl BaseUnitType for Kg {}
impl MassUnitType for Kg {}

#[derive(Clone, Copy, Default)]
pub struct Radian;
impl BaseUnitType for Radian {}
impl AngleUnitType for Radian {}

#[derive(Clone, Copy, Default)]
pub struct Degree;
impl BaseUnitType for Degree {}
impl AngleUnitType for Degree {}

pub trait TimeBaseUnit: DimensionType {}
pub trait LengthBaseUnit: DimensionType {}
pub trait MassBaseUnit: DimensionType {}
pub trait AngleBaseUnit: DimensionType {}

impl<S, I, U> TimeBaseUnit for BaseUnit<S, I, U> where S: SignType, I: NotZeroIdentityType, U: TimeUnitType, BaseUnit<S, I, U>: DimensionType {}
impl<S, I, U> LengthBaseUnit for BaseUnit<S, I, U> where S: SignType, I: NotZeroIdentityType, U: LengthUnitType, BaseUnit<S, I, U>: DimensionType {}
impl<S, I, U> MassBaseUnit for BaseUnit<S, I, U> where S: SignType, I: NotZeroIdentityType, U: MassUnitType, BaseUnit<S, I, U>: DimensionType {}
impl<S, I, U> AngleBaseUnit for BaseUnit<S, I, U> where S: SignType, I: NotZeroIdentityType, U: AngleUnitType, BaseUnit<S, I, U>: DimensionType {}

impl TimeBaseUnit for Unitless {}
impl LengthBaseUnit for Unitless {}
impl MassBaseUnit for Unitless {}
impl AngleBaseUnit for Unitless {}

#[derive(Clone, Copy)]
pub struct Unit<T, L, M, A> (pub T, pub L, pub M, pub A) where
T: TimeBaseUnit,
L: LengthBaseUnit,
M: MassBaseUnit,
A: AngleBaseUnit;

impl<T, L, M, A> Default for Unit<T, L, M, A> where
T: TimeBaseUnit + Default,
L: LengthBaseUnit + Default,
M: MassBaseUnit + Default,
A: AngleBaseUnit + Default
{
    fn default() -> Self {
        Self(T::default(), L::default(), M::default(), A::default())
    }
}

pub struct UnitOperation<U1, U2> (pub U1, pub U2);
/* (pub (U1, U2), pub (T, L, M, A))where
T: TimeBaseUnit,
L: LengthBaseUnit,
M: MassBaseUnit,
A: AngleBaseUnit; */

pub trait Unify {
    type UnifyOutput;
    fn unify(self) -> Self::UnifyOutput;
}

impl<T1, L1, M1, A1, T2, L2, M2, A2> Unify for UnitOperation<Unit<T1, L1, M1, A1>, Unit<T2, L2, M2, A2>>
where
T1: TimeBaseUnit, T2: TimeBaseUnit,
L1: LengthBaseUnit, L2: LengthBaseUnit,
M1: MassBaseUnit, M2: MassBaseUnit,
A1: AngleBaseUnit, A2: AngleBaseUnit,
O<T1, T2>: TypeUnify,
O<L1, L2>: TypeUnify,
O<M1, M2>: TypeUnify,
O<A1, A2>: TypeUnify,
<O<T1, T2> as TypeUnify>::UnifyOutput: TimeBaseUnit,
<O<L1, L2> as TypeUnify>::UnifyOutput: LengthBaseUnit,
<O<M1, M2> as TypeUnify>::UnifyOutput: MassBaseUnit,
<O<A1, A2> as TypeUnify>::UnifyOutput: AngleBaseUnit,
{
    type UnifyOutput = Unit<
        <O<T1, T2> as TypeUnify>::UnifyOutput,
        <O<L1, L2> as TypeUnify>::UnifyOutput,
        <O<M1, M2> as TypeUnify>::UnifyOutput,
        <O<A1, A2> as TypeUnify>::UnifyOutput,
    >;
    fn unify(self) -> Self::UnifyOutput {
        Unit(
            O(self.0.0, self.1.0).unify(),
            O(self.0.1, self.1.1).unify(),
            O(self.0.2, self.1.2).unify(),
            O(self.0.3, self.1.3).unify()
        )
    }
}

fn unit_operate<T1, L1, M1, A1, T2, L2, M2, A2>(u1: Unit<T1, L1, M1, A1>, u2: Unit<T2, L2, M2, A2>)
-> UnitOperation<Unit<T1, L1, M1, A1>, Unit<T2, L2, M2, A2>>
where
T1: TimeBaseUnit, T2: TimeBaseUnit,
L1: LengthBaseUnit, L2: LengthBaseUnit,
M1: MassBaseUnit, M2: MassBaseUnit,
A1: AngleBaseUnit, A2: AngleBaseUnit,
UnitOperation<Unit<T1, L1, M1, A1>, Unit<T2, L2, M2, A2>>: Unify
{
    UnitOperation(u1, u2)
}
