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
use super::units::{Unit, MulUnit, DivUnit};

#[derive(Clone, Copy)]
pub struct PhysicalQuantity<T, U>(pub T, pub U);

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
    ValueType: std::ops::Add<Output = ValueType>
> std::ops::Add for PhysicalQuantity<ValueType, Unit<
    T, T_D,
    L, L_D,
    M, M_D,
    I, I_D,
    TMP, TMP_D,
    SA, SA_D,
    LI, LI_D,
    A, A_D,
    SR, SR_D,
>> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Self(v1, u) = self;
        let Self(v2, _) = rhs;
        Self(v1+v2, u)
    }
}

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
    ValueType: std::ops::Sub<Output = ValueType>
> std::ops::Sub for PhysicalQuantity<ValueType, Unit<
    T, T_D,
    L, L_D,
    M, M_D,
    I, I_D,
    TMP, TMP_D,
    SA, SA_D,
    LI, LI_D,
    A, A_D,
    SR, SR_D,
>> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let Self(v1, u) = self;
        let Self(v2, _) = rhs;
        Self(v1-v2, u)
    }
}

pub struct MulPhysicalQuantity<
    ValueType1: std::ops::Mul<ValueType2>, ValueType2,
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
>(<ValueType1 as std::ops::Mul<ValueType2>>::Output, MulUnit<
    T, T_D1, T_D2,
    L, L_D1, L_D2,
    M, M_D1, M_D2,
    I, I_D1, I_D2,
    TMP, TMP_D1, TMP_D2,
    SA, SA_D1, SA_D2,
    LI, LI_D1, LI_D2,
    A, A_D1, A_D2,
    SR, SR_D1, SR_D2,
>);

impl<
    ValueType1: std::ops::Mul<ValueType2>, ValueType2,
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
> MulPhysicalQuantity<
    ValueType1, ValueType2,
    T, T_D1, T_D2,
    L, L_D1, L_D2,
    M, M_D1, M_D2,
    I, I_D1, I_D2,
    TMP, TMP_D1, TMP_D2,
    SA, SA_D1, SA_D2,
    LI, LI_D1, LI_D2,
    A, A_D1, A_D2,
    SR, SR_D1, SR_D2,
>where
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
    pub fn unify(self) -> PhysicalQuantity<<ValueType1 as std::ops::Mul<ValueType2>>::Output, Unit<
        T, <O<T_D1, T_D2> as Simplify>::Output,
        L, <O<L_D1, L_D2> as Simplify>::Output,
        M, <O<M_D1, M_D2> as Simplify>::Output,
        I, <O<I_D1, I_D2> as Simplify>::Output,
        TMP, <O<TMP_D1, TMP_D2> as Simplify>::Output,
        SA, <O<SA_D1, SA_D2> as Simplify>::Output,
        LI, <O<LI_D1, LI_D2> as Simplify>::Output,
        A, <O<A_D1, A_D2> as Simplify>::Output,
        SR, <O<SR_D1, SR_D2> as Simplify>::Output,
    >> {
        let Self(v, u) = self;
        PhysicalQuantity(v, u.unify())
    }
}

impl<
    ValueType1: std::ops::Mul<ValueType2>, ValueType2,
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
> std::ops::Mul<PhysicalQuantity<ValueType2, Unit<
    T, T_D2,
    L, L_D2,
    M, M_D2,
    I, I_D2,
    TMP, TMP_D2,
    SA, SA_D2,
    LI, LI_D2,
    A, A_D2,
    SR, SR_D2,
>>> for PhysicalQuantity<ValueType1, Unit<
    T, T_D1,
    L, L_D1,
    M, M_D1,
    I, I_D1,
    TMP, TMP_D1,
    SA, SA_D1,
    LI, LI_D1,
    A, A_D1,
    SR, SR_D1,
>> {
    type Output = MulPhysicalQuantity<
        ValueType1, ValueType2,
        T, T_D1, T_D2,
        L, L_D1, L_D2,
        M, M_D1, M_D2,
        I, I_D1, I_D2,
        TMP, TMP_D1, TMP_D2,
        SA, SA_D1, SA_D2,
        LI, LI_D1, LI_D2,
        A, A_D1, A_D2,
        SR, SR_D1, SR_D2,
    >;
    fn mul(self, rhs: PhysicalQuantity<ValueType2, Unit<
        T, T_D2,
        L, L_D2,
        M, M_D2,
        I, I_D2,
        TMP, TMP_D2,
        SA, SA_D2,
        LI, LI_D2,
        A, A_D2,
        SR, SR_D2,
    >>) -> Self::Output {
        let PhysicalQuantity(v1, u1) = self;
        let PhysicalQuantity(v2, u2) = rhs;
        MulPhysicalQuantity(v1 * v2, MulUnit(u1, u2))
    }
}

pub struct DivPhysicalQuantity<
    ValueType1: std::ops::Div<ValueType2>, ValueType2,
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
>(<ValueType1 as std::ops::Div<ValueType2>>::Output, DivUnit<
    T, T_D1, T_D2,
    L, L_D1, L_D2,
    M, M_D1, M_D2,
    I, I_D1, I_D2,
    TMP, TMP_D1, TMP_D2,
    SA, SA_D1, SA_D2,
    LI, LI_D1, LI_D2,
    A, A_D1, A_D2,
    SR, SR_D1, SR_D2,
>);

impl<
    ValueType1: std::ops::Div<ValueType2>, ValueType2,
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
> DivPhysicalQuantity<
    ValueType1, ValueType2,
    T, T_D1, T_D2,
    L, L_D1, L_D2,
    M, M_D1, M_D2,
    I, I_D1, I_D2,
    TMP, TMP_D1, TMP_D2,
    SA, SA_D1, SA_D2,
    LI, LI_D1, LI_D2,
    A, A_D1, A_D2,
    SR, SR_D1, SR_D2,
>where
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
    pub fn unify(self) -> PhysicalQuantity<<ValueType1 as std::ops::Div<ValueType2>>::Output, Unit<
        T, <O<T_D1, <T_D2 as Dimension>::Inverse> as Simplify>::Output,
        L, <O<L_D1, <L_D2 as Dimension>::Inverse> as Simplify>::Output,
        M, <O<M_D1, <M_D2 as Dimension>::Inverse> as Simplify>::Output,
        I, <O<I_D1, <I_D2 as Dimension>::Inverse> as Simplify>::Output,
        TMP, <O<TMP_D1, <TMP_D2 as Dimension>::Inverse> as Simplify>::Output,
        SA, <O<SA_D1, <SA_D2 as Dimension>::Inverse> as Simplify>::Output,
        LI, <O<LI_D1, <LI_D2 as Dimension>::Inverse> as Simplify>::Output,
        A, <O<A_D1, <A_D2 as Dimension>::Inverse> as Simplify>::Output,
        SR, <O<SR_D1, <SR_D2 as Dimension>::Inverse> as Simplify>::Output,
    >> {
        let Self(v, u) = self;
        PhysicalQuantity(v, u.unify())
    }
}

impl<
    ValueType1: std::ops::Div<ValueType2>, ValueType2,
    T: TimeUnit, T_D1: Dimension, T_D2: Dimension,
    L: LengthUnit, L_D1: Dimension, L_D2: Dimension,
    M: MassUnit, M_D1: Dimension, M_D2: Dimension,
    I: CurrentUnit, I_D1: Dimension, I_D2: Dimension,
    TMP: TemperatureUnit, TMP_D1: Dimension, TMP_D2: Dimension,
    SA: SubstanceAmountUnit, SA_D1: Dimension, SA_D2: Dimension,
    LI: LuminousIntensityUnit, LI_D1: Dimension, LI_D2: Dimension,
    A: AngleUnit, A_D1: Dimension, A_D2: Dimension,
    SR: SolidAngleUnit, SR_D1: Dimension, SR_D2: Dimension,
> std::ops::Div<PhysicalQuantity<ValueType2, Unit<
    T, T_D2,
    L, L_D2,
    M, M_D2,
    I, I_D2,
    TMP, TMP_D2,
    SA, SA_D2,
    LI, LI_D2,
    A, A_D2,
    SR, SR_D2,
>>> for PhysicalQuantity<ValueType1, Unit<
    T, T_D1,
    L, L_D1,
    M, M_D1,
    I, I_D1,
    TMP, TMP_D1,
    SA, SA_D1,
    LI, LI_D1,
    A, A_D1,
    SR, SR_D1,
>> {
    type Output = DivPhysicalQuantity<
        ValueType1, ValueType2,
        T, T_D1, T_D2,
        L, L_D1, L_D2,
        M, M_D1, M_D2,
        I, I_D1, I_D2,
        TMP, TMP_D1, TMP_D2,
        SA, SA_D1, SA_D2,
        LI, LI_D1, LI_D2,
        A, A_D1, A_D2,
        SR, SR_D1, SR_D2,
    >;
    fn div(self, rhs: PhysicalQuantity<ValueType2, Unit<
        T, T_D2,
        L, L_D2,
        M, M_D2,
        I, I_D2,
        TMP, TMP_D2,
        SA, SA_D2,
        LI, LI_D2,
        A, A_D2,
        SR, SR_D2,
    >>) -> Self::Output {
        let PhysicalQuantity(v1, u1) = self;
        let PhysicalQuantity(v2, u2) = rhs;
        DivPhysicalQuantity(v1 / v2, DivUnit(u1, u2))
    }
}
