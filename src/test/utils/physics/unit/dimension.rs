pub trait IdentityType {}

pub trait NotZeroIdentityType: IdentityType {}

impl IdentityType for () {}

#[derive(Clone, Copy)]
pub struct I<S: IdentityType>(S);

impl<S: IdentityType> IdentityType for I<S> {}

impl<S: IdentityType> NotZeroIdentityType for I<S> {}

impl Default for I<()> {
    fn default() -> Self {
        Self(())
    }
}

impl<S: NotZeroIdentityType + Default> Default for I<S> {
    fn default() -> Self {
        Self(S::default())
    }
}

#[derive(Clone, Copy, Default)]
pub struct Positive;
#[derive(Clone, Copy, Default)]
pub struct Negative;

pub trait SignType {}

impl SignType for Positive {}
impl SignType for Negative {}

pub trait DimensionType {
    type Inverse;
    fn inverse(self) -> Self::Inverse;
}

pub trait NotZeroDimensionType: DimensionType {}

#[derive(Clone, Copy, Default)]
pub struct Unitless;

impl DimensionType for Unitless {
    type Inverse = Unitless;
    fn inverse(self) -> Self::Inverse {
        Unitless
    }
}

#[derive(Clone, Copy)]
pub struct Dimension<S, I> (S, I) where S: SignType, I: NotZeroIdentityType;

impl<S, I> Default for Dimension<S, I> where S: SignType + Default, I: NotZeroIdentityType + Default {
    fn default() -> Self {
        Self(S::default(), I::default())
    }
}

impl<I: NotZeroIdentityType> DimensionType for Dimension<Positive, I> {
    type Inverse = Dimension<Negative, I>;
    fn inverse(self) -> Self::Inverse {
        Dimension(Negative, self.1)
    }
}

impl<I: NotZeroIdentityType> NotZeroDimensionType for Dimension<Positive, I> {}

impl<I: NotZeroIdentityType> DimensionType for Dimension<Negative, I> {
    type Inverse = Dimension<Positive, I>;
    fn inverse(self) -> Self::Inverse {
        Dimension(Positive, self.1)
    }
}

impl<I: NotZeroIdentityType> NotZeroDimensionType for Dimension<Negative, I> {}


pub trait TypeUnify {
    type UnifyOutput;
    fn unify(self) -> Self::UnifyOutput;
}

#[derive(Clone, Copy)]
pub struct O<D1, D2> (pub D1, pub D2) where D1: DimensionType, D2: DimensionType;

impl TypeUnify for O<Unitless, Unitless> {
    type UnifyOutput = Unitless;
    fn unify(self) -> Self::UnifyOutput {
        Unitless
    }
}

impl<D: NotZeroDimensionType> TypeUnify for O<Unitless, D> {
    type UnifyOutput = D;
    fn unify(self) -> Self::UnifyOutput {
        self.1
    }
}

impl<D: NotZeroDimensionType> TypeUnify for O<D, Unitless> {
    type UnifyOutput = D;
    fn unify(self) -> Self::UnifyOutput {
        self.0
    }
}

impl<S> TypeUnify for O<Dimension<Positive, I<S>>, Dimension<Positive, I<()>>> where S: IdentityType {
    type UnifyOutput = Dimension<Positive, I<I<S>>>;
    fn unify(self) -> Self::UnifyOutput {
        Dimension(Positive, I(self.0.1))
    }
}

impl<S1, S2> TypeUnify for O<Dimension<Positive, I<S1>>, Dimension<Positive, I<S2>>>
where S1: IdentityType, S2: NotZeroIdentityType,
O<Dimension<Positive, I<I<S1>>>, Dimension<Positive, S2>>: TypeUnify
{
    type UnifyOutput = <O<Dimension<Positive, I<I<S1>>>, Dimension<Positive, S2>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(Dimension(Positive, I(self.0.1)), Dimension(Positive, self.1.1.0)).unify()
    }
}

impl<S> TypeUnify for O<Dimension<Negative, I<S>>, Dimension<Negative, I<()>>> where S: IdentityType {
    type UnifyOutput = Dimension<Negative, I<I<S>>>;
    fn unify(self) -> Self::UnifyOutput {
        Dimension(Negative, I(self.0.1))
    }
}

impl<S1, S2> TypeUnify for O<Dimension<Negative, I<S1>>, Dimension<Negative, I<S2>>>
where S1: IdentityType, S2: NotZeroIdentityType,
O<Dimension<Negative, I<I<S1>>>, Dimension<Negative, S2>>: TypeUnify
{
    type UnifyOutput = <O<Dimension<Negative, I<I<S1>>>, Dimension<Negative, S2>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(Dimension(Negative, I(self.0.1)), Dimension(Negative, self.1.1.0)).unify()
    }
}

impl<S1, S2> TypeUnify for O<Dimension<Positive, I<S1>>, Dimension<Negative, I<S2>>>
where S1: NotZeroIdentityType, S2: NotZeroIdentityType,
O<Dimension<Positive, S1>, Dimension<Negative, S2>>: TypeUnify
{
    type UnifyOutput = <O<Dimension<Positive, S1>, Dimension<Negative, S2>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(Dimension(Positive, self.0.1.0), Dimension(Negative, self.1.1.0)).unify()
    }
}

impl<S1, S2> TypeUnify for O<Dimension<Negative, I<S1>>, Dimension<Positive, I<S2>>>
where S1: NotZeroIdentityType, S2: NotZeroIdentityType,
O<Dimension<Negative, S1>, Dimension<Positive, S2>>: TypeUnify
{
    type UnifyOutput = <O<Dimension<Negative, S1>, Dimension<Positive, S2>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(Dimension(Negative, self.0.1.0), Dimension(Positive, self.1.1.0)).unify()
    }
}

impl<S> TypeUnify for O<Dimension<Positive, I<S>>, Dimension<Negative, I<()>>> where S: NotZeroIdentityType {
    type UnifyOutput = Dimension<Positive, S>;
    fn unify(self) -> Self::UnifyOutput {
        Dimension(Positive, self.0.1.0)
    }
}

impl<S> TypeUnify for O<Dimension<Negative, I<S>>, Dimension<Positive, I<()>>> where S: NotZeroIdentityType {
    type UnifyOutput = Dimension<Negative, S>;
    fn unify(self) -> Self::UnifyOutput {
        Dimension(Negative, self.0.1.0)
    }
}

impl<S> TypeUnify for O<Dimension<Positive, I<()>>, Dimension<Negative, I<S>>> where S: NotZeroIdentityType {
    type UnifyOutput = Dimension<Negative, S>;
    fn unify(self) -> Self::UnifyOutput {
        Dimension(Negative, self.1.1.0)
    }
}

impl<S> TypeUnify for O<Dimension<Negative, I<()>>, Dimension<Positive, I<S>>> where S: NotZeroIdentityType {
    type UnifyOutput = Dimension<Positive, S>;
    fn unify(self) -> Self::UnifyOutput {
        Dimension(Positive, self.1.1.0)
    }
}

impl TypeUnify for O<Dimension<Positive, I<()>>, Dimension<Negative, I<()>>> {
    type UnifyOutput = Unitless;
    fn unify(self) -> Self::UnifyOutput {
        Unitless
    }
}

impl TypeUnify for O<Dimension<Negative, I<()>>, Dimension<Positive, I<()>>> {
    type UnifyOutput = Unitless;
    fn unify(self) -> Self::UnifyOutput {
        Unitless
    }
}

pub type I1 = I<()>;
pub type I2 = I<I1>;
pub type I3 = I<I2>;
pub type I4 = I<I3>;