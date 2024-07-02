use super::dimension::{P, N, Unitless, O, Simplify, Dimension};
use super::si::{self,
    TimeUnit, Second,
    LengthUnit, Meter,
    MassUnit, Kg,
    CurrentUnit, Ampere,
    TemperatureUnit, Kelvin,
    SubstanceAmountUnit, Mole,
    LuminousIntensityUnit, Candela,
    AngleUnit, Radian,
    SolidAngleUnit, Steradian,
};

pub trait BaseUnitType: Copy {
    const Value: Self;
}

#[derive(Clone, Copy)]
pub struct BaseUnit<T: BaseUnitType, D: Dimension>(T, D);

impl<T: BaseUnitType, D: Dimension> BaseUnit<T, D> {
    pub const Value: Self = Self(T::Value, D::Value);
}

pub struct MulBaseUnit<T: BaseUnitType, D1: Dimension, D2: Dimension>(pub BaseUnit<T, D1>, pub BaseUnit<T, D2>);

impl<T: BaseUnitType, D1: Dimension, D2: Dimension> MulBaseUnit<T, D1, D2>
where
    O<D1, D2>: Simplify,
    <O<D1, D2> as Simplify>::Output: Dimension
{
    pub fn unify(self) -> BaseUnit<T, <O<D1, D2> as Simplify>::Output> {
        BaseUnit(T::Value, O(D1::Value, D2::Value).simplify())
    }
}

pub struct DivBaseUnit<T: BaseUnitType, D1: Dimension, D2: Dimension>(pub BaseUnit<T, D1>, pub BaseUnit<T, D2>);

impl<T: BaseUnitType, D1: Dimension, D2: Dimension> DivBaseUnit<T, D1, D2>
where
    O<D1, <D2 as Dimension>::Inverse>: Simplify,
    <O<D1, <D2 as Dimension>::Inverse> as Simplify>::Output: Dimension
{
    pub fn unify(self) -> BaseUnit<T, <O<D1, <D2 as Dimension>::Inverse> as Simplify>::Output> {
        BaseUnit(T::Value, O(D1::Value, D2::Value.inverse()).simplify())
    }
}

#[derive(Clone, Copy)]
pub struct Unit<
    T: TimeUnit, T_D: Dimension,
    L: LengthUnit, L_D: Dimension,
    M: MassUnit, M_D: Dimension,
    I: CurrentUnit, I_D: Dimension,
    TMP: TemperatureUnit, TMP_D: Dimension,
    SA: SubstanceAmountUnit, SA_D: Dimension,
    LI: LuminousIntensityUnit, LI_D: Dimension,
    A: AngleUnit, A_D: Dimension,
    SR: SolidAngleUnit, SR_D: Dimension,
>(
    BaseUnit<T, T_D>,
    BaseUnit<L, L_D>,
    BaseUnit<M, M_D>,
    BaseUnit<I, I_D>,
    BaseUnit<TMP, TMP_D>,
    BaseUnit<SA, SA_D>,
    BaseUnit<LI, LI_D>,
    BaseUnit<A, A_D>,
    BaseUnit<SR, SR_D>,
);

impl<
    T: TimeUnit, T_D: Dimension,
    L: LengthUnit, L_D: Dimension,
    M: MassUnit, M_D: Dimension,
    I: CurrentUnit, I_D: Dimension,
    TMP: TemperatureUnit, TMP_D: Dimension,
    SA: SubstanceAmountUnit, SA_D: Dimension,
    LI: LuminousIntensityUnit, LI_D: Dimension,
    A: AngleUnit, A_D: Dimension,
    SR: SolidAngleUnit, SR_D: Dimension,
> Unit<
    T, T_D,
    L, L_D,
    M, M_D,
    I, I_D,
    TMP, TMP_D,
    SA, SA_D,
    LI, LI_D,
    A, A_D,
    SR, SR_D,
> {
    pub const Value: Self = Self(
        BaseUnit(T::Value, T_D::Value),
        BaseUnit(L::Value, L_D::Value),
        BaseUnit(M::Value, M_D::Value),
        BaseUnit(I::Value, I_D::Value),
        BaseUnit(TMP::Value, TMP_D::Value),
        BaseUnit(SA::Value, SA_D::Value),
        BaseUnit(LI::Value, LI_D::Value),
        BaseUnit(A::Value, A_D::Value),
        BaseUnit(SR::Value, SR_D::Value),
    );
}

pub struct MulUnit<
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
>(pub Unit<
    T, T_D1,
    L, L_D1,
    M, M_D1,
    I, I_D1,
    TMP, TMP_D1,
    SA, SA_D1,
    LI, LI_D1,
    A, A_D1,
    SR, SR_D1,
>, pub Unit<
    T, T_D2,
    L, L_D2,
    M, M_D2,
    I, I_D2,
    TMP, TMP_D2,
    SA, SA_D2,
    LI, LI_D2,
    A, A_D2,
    SR, SR_D2,
>);

impl<
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
> MulUnit<
    T, T_D1, T_D2,
    L, L_D1, L_D2,
    M, M_D1, M_D2,
    I, I_D1, I_D2,
    TMP, TMP_D1, TMP_D2,
    SA, SA_D1, SA_D2,
    LI, LI_D1, LI_D2,
    A, A_D1, A_D2,
    SR, SR_D1, SR_D2,
> where
    O<T_D1, T_D2>: Simplify,
    O<L_D1, L_D2>: Simplify,
    O<M_D1, M_D2>: Simplify,
    O<I_D1, I_D2>: Simplify,
    O<TMP_D1, TMP_D2>: Simplify,
    O<SA_D1, SA_D2>: Simplify,
    O<LI_D1, LI_D2>: Simplify,
    O<A_D1, A_D2>: Simplify,
    O<SR_D1, SR_D2>: Simplify,
    <O<T_D1, T_D2> as Simplify>::Output: Dimension,
    <O<L_D1, L_D2> as Simplify>::Output: Dimension,
    <O<M_D1, M_D2> as Simplify>::Output: Dimension,
    <O<I_D1, I_D2> as Simplify>::Output: Dimension,
    <O<TMP_D1, TMP_D2> as Simplify>::Output: Dimension,
    <O<SA_D1, SA_D2> as Simplify>::Output: Dimension,
    <O<LI_D1, LI_D2> as Simplify>::Output: Dimension,
    <O<A_D1, A_D2> as Simplify>::Output: Dimension,
    <O<SR_D1, SR_D2> as Simplify>::Output: Dimension,
{
    pub fn unify(self) -> Unit<
        T, <O<T_D1, T_D2> as Simplify>::Output,
        L, <O<L_D1, L_D2> as Simplify>::Output,
        M, <O<M_D1, M_D2> as Simplify>::Output,
        I, <O<I_D1, I_D2> as Simplify>::Output,
        TMP, <O<TMP_D1, TMP_D2> as Simplify>::Output,
        SA, <O<SA_D1, SA_D2> as Simplify>::Output,
        LI, <O<LI_D1, LI_D2> as Simplify>::Output,
        A, <O<A_D1, A_D2> as Simplify>::Output,
        SR, <O<SR_D1, SR_D2> as Simplify>::Output,
    > {
        Unit(

            MulBaseUnit(BaseUnit(T::Value, T_D1::Value), BaseUnit(T::Value, T_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(L::Value, L_D1::Value), BaseUnit(L::Value, L_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(M::Value, M_D1::Value), BaseUnit(M::Value, M_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(I::Value, I_D1::Value), BaseUnit(I::Value, I_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(TMP::Value, TMP_D1::Value), BaseUnit(TMP::Value, TMP_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(SA::Value, SA_D1::Value), BaseUnit(SA::Value, SA_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(LI::Value, LI_D1::Value), BaseUnit(LI::Value, LI_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(A::Value, A_D1::Value), BaseUnit(A::Value, A_D2::Value)).unify(),
            MulBaseUnit(BaseUnit(SR::Value, SR_D1::Value), BaseUnit(SR::Value, SR_D2::Value)).unify(),
        )
    }
}

pub struct DivUnit<
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
>(pub Unit<
    T, T_D1,
    L, L_D1,
    M, M_D1,
    I, I_D1,
    TMP, TMP_D1,
    SA, SA_D1,
    LI, LI_D1,
    A, A_D1,
    SR, SR_D1,
>, pub Unit<
    T, T_D2,
    L, L_D2,
    M, M_D2,
    I, I_D2,
    TMP, TMP_D2,
    SA, SA_D2,
    LI, LI_D2,
    A, A_D2,
    SR, SR_D2,
>);

impl<
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
> DivUnit<
    T, T_D1, T_D2,
    L, L_D1, L_D2,
    M, M_D1, M_D2,
    I, I_D1, I_D2,
    TMP, TMP_D1, TMP_D2,
    SA, SA_D1, SA_D2,
    LI, LI_D1, LI_D2,
    A, A_D1, A_D2,
    SR, SR_D1, SR_D2,
> where
    O<T_D1, <T_D2 as Dimension>::Inverse>: Simplify,
    O<L_D1, <L_D2 as Dimension>::Inverse>: Simplify,
    O<M_D1, <M_D2 as Dimension>::Inverse>: Simplify,
    O<I_D1, <I_D2 as Dimension>::Inverse>: Simplify,
    O<TMP_D1, <TMP_D2 as Dimension>::Inverse>: Simplify,
    O<SA_D1, <SA_D2 as Dimension>::Inverse>: Simplify,
    O<LI_D1, <LI_D2 as Dimension>::Inverse>: Simplify,
    O<A_D1, <A_D2 as Dimension>::Inverse>: Simplify,
    O<SR_D1, <SR_D2 as Dimension>::Inverse>: Simplify,
    <O<T_D1, <T_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<L_D1, <L_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<M_D1, <M_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<I_D1, <I_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<TMP_D1, <TMP_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<SA_D1, <SA_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<LI_D1, <LI_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<A_D1, <A_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
    <O<SR_D1, <SR_D2 as Dimension>::Inverse> as Simplify>::Output: Dimension,
{
    pub fn unify(self) -> Unit<
        T, <O<T_D1, <T_D2 as Dimension>::Inverse> as Simplify>::Output,
        L, <O<L_D1, <L_D2 as Dimension>::Inverse> as Simplify>::Output,
        M, <O<M_D1, <M_D2 as Dimension>::Inverse> as Simplify>::Output,
        I, <O<I_D1, <I_D2 as Dimension>::Inverse> as Simplify>::Output,
        TMP, <O<TMP_D1, <TMP_D2 as Dimension>::Inverse> as Simplify>::Output,
        SA, <O<SA_D1, <SA_D2 as Dimension>::Inverse> as Simplify>::Output,
        LI, <O<LI_D1, <LI_D2 as Dimension>::Inverse> as Simplify>::Output,
        A, <O<A_D1, <A_D2 as Dimension>::Inverse> as Simplify>::Output,
        SR, <O<SR_D1, <SR_D2 as Dimension>::Inverse> as Simplify>::Output,
    > {
        Unit(

            DivBaseUnit(BaseUnit(T::Value, T_D1::Value), BaseUnit(T::Value, T_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(L::Value, L_D1::Value), BaseUnit(L::Value, L_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(M::Value, M_D1::Value), BaseUnit(M::Value, M_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(I::Value, I_D1::Value), BaseUnit(I::Value, I_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(TMP::Value, TMP_D1::Value), BaseUnit(TMP::Value, TMP_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(SA::Value, SA_D1::Value), BaseUnit(SA::Value, SA_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(LI::Value, LI_D1::Value), BaseUnit(LI::Value, LI_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(A::Value, A_D1::Value), BaseUnit(A::Value, A_D2::Value)).unify(),
            DivBaseUnit(BaseUnit(SR::Value, SR_D1::Value), BaseUnit(SR::Value, SR_D2::Value)).unify(),
        )
    }
}

pub type SIUnit<
    T_D,
    L_D,
    M_D,
    I_D,
    TMP_D,
    SA_D,
    LI_D,
    A_D,
    SR_D,
> = Unit<
    Second, T_D,
    Meter, L_D,
    Kg, M_D,
    Ampere, I_D,
    Kelvin, TMP_D,
    Mole, SA_D,
    Candela, LI_D,
    Radian, A_D,
    Steradian, SR_D,
>;
