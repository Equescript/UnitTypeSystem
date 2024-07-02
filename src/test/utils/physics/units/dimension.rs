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
pub struct BaseUnit<S, I, U> (S, I, U) where S: SignType, I: NotZeroIdentityType;

impl<S, I, U> Default for BaseUnit<S, I, U> where S: SignType + Default, I: NotZeroIdentityType + Default, U: Default{
    fn default() -> Self {
        Self(S::default(), I::default(), U::default())
    }
}

impl<I: NotZeroIdentityType, U> DimensionType for BaseUnit<Positive, I, U> {
    type Inverse = BaseUnit<Negative, I, U>;
    fn inverse(self) -> Self::Inverse {
        BaseUnit(Negative, self.1, self.2)
    }
}

impl<I: NotZeroIdentityType, U> NotZeroDimensionType for BaseUnit<Positive, I, U> {}

impl<I: NotZeroIdentityType, U> DimensionType for BaseUnit<Negative, I, U> {
    type Inverse = BaseUnit<Positive, I, U>;
    fn inverse(self) -> Self::Inverse {
        BaseUnit(Positive, self.1, self.2)
    }
}

impl<I: NotZeroIdentityType, U> NotZeroDimensionType for BaseUnit<Negative, I, U> {}


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

impl<S, T> TypeUnify for O<BaseUnit<Positive, I<S>, T>, BaseUnit<Positive, I<()>, T>> where S: IdentityType {
    type UnifyOutput = BaseUnit<Positive, I<I<S>>, T>;
    fn unify(self) -> Self::UnifyOutput {
        BaseUnit(Positive, I(self.0.1), self.0.2)
    }
}

impl<S1, S2, T> TypeUnify for O<BaseUnit<Positive, I<S1>, T>, BaseUnit<Positive, I<S2>, T>>
where S1: IdentityType, S2: NotZeroIdentityType,
O<BaseUnit<Positive, I<I<S1>>, T>, BaseUnit<Positive, S2, T>>: TypeUnify
{
    type UnifyOutput = <O<BaseUnit<Positive, I<I<S1>>, T>, BaseUnit<Positive, S2, T>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(BaseUnit(Positive, I(self.0.1), self.0.2), BaseUnit(Positive, self.1.1.0, self.1.2)).unify()
    }
}

impl<S, T> TypeUnify for O<BaseUnit<Negative, I<S>, T>, BaseUnit<Negative, I<()>, T>> where S: IdentityType {
    type UnifyOutput = BaseUnit<Negative, I<I<S>>, T>;
    fn unify(self) -> Self::UnifyOutput {
        BaseUnit(Negative, I(self.0.1), self.0.2)
    }
}

impl<S1, S2, T> TypeUnify for O<BaseUnit<Negative, I<S1>, T>, BaseUnit<Negative, I<S2>, T>>
where S1: IdentityType, S2: NotZeroIdentityType,
O<BaseUnit<Negative, I<I<S1>>, T>, BaseUnit<Negative, S2, T>>: TypeUnify
{
    type UnifyOutput = <O<BaseUnit<Negative, I<I<S1>>, T>, BaseUnit<Negative, S2, T>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(BaseUnit(Negative, I(self.0.1), self.0.2), BaseUnit(Negative, self.1.1.0, self.1.2)).unify()
    }
}

impl<S1, S2, T> TypeUnify for O<BaseUnit<Positive, I<S1>, T>, BaseUnit<Negative, I<S2>, T>>
where S1: NotZeroIdentityType, S2: NotZeroIdentityType,
O<BaseUnit<Positive, S1, T>, BaseUnit<Negative, S2, T>>: TypeUnify
{
    type UnifyOutput = <O<BaseUnit<Positive, S1, T>, BaseUnit<Negative, S2, T>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(BaseUnit(Positive, self.0.1.0, self.0.2), BaseUnit(Negative, self.1.1.0, self.1.2)).unify()
    }
}

impl<S1, S2, T> TypeUnify for O<BaseUnit<Negative, I<S1>, T>, BaseUnit<Positive, I<S2>, T>>
where S1: NotZeroIdentityType, S2: NotZeroIdentityType,
O<BaseUnit<Negative, S1, T>, BaseUnit<Positive, S2, T>>: TypeUnify
{
    type UnifyOutput = <O<BaseUnit<Negative, S1, T>, BaseUnit<Positive, S2, T>> as TypeUnify>::UnifyOutput;
    fn unify(self) -> Self::UnifyOutput {
        O(BaseUnit(Negative, self.0.1.0, self.0.2), BaseUnit(Positive, self.1.1.0, self.1.2)).unify()
    }
}

impl<S, T> TypeUnify for O<BaseUnit<Positive, I<S>, T>, BaseUnit<Negative, I<()>, T>> where S: NotZeroIdentityType {
    type UnifyOutput = BaseUnit<Positive, S, T>;
    fn unify(self) -> Self::UnifyOutput {
        BaseUnit(Positive, self.0.1.0, self.0.2)
    }
}

impl<S, T> TypeUnify for O<BaseUnit<Negative, I<S>, T>, BaseUnit<Positive, I<()>, T>> where S: NotZeroIdentityType {
    type UnifyOutput = BaseUnit<Negative, S, T>;
    fn unify(self) -> Self::UnifyOutput {
        BaseUnit(Negative, self.0.1.0, self.0.2)
    }
}

impl<S, T> TypeUnify for O<BaseUnit<Positive, I<()>, T>, BaseUnit<Negative, I<S>, T>> where S: NotZeroIdentityType {
    type UnifyOutput = BaseUnit<Negative, S, T>;
    fn unify(self) -> Self::UnifyOutput {
        BaseUnit(Negative, self.1.1.0, self.1.2)
    }
}

impl<S, T> TypeUnify for O<BaseUnit<Negative, I<()>, T>, BaseUnit<Positive, I<S>, T>> where S: NotZeroIdentityType {
    type UnifyOutput = BaseUnit<Positive, S, T>;
    fn unify(self) -> Self::UnifyOutput {
        BaseUnit(Positive, self.1.1.0, self.1.2)
    }
}

impl<T> TypeUnify for O<BaseUnit<Positive, I<()>, T>, BaseUnit<Negative, I<()>, T>> {
    type UnifyOutput = Unitless;
    fn unify(self) -> Self::UnifyOutput {
        Unitless
    }
}

impl<T> TypeUnify for O<BaseUnit<Negative, I<()>, T>, BaseUnit<Positive, I<()>, T>> {
    type UnifyOutput = Unitless;
    fn unify(self) -> Self::UnifyOutput {
        Unitless
    }
}

pub type I1 = I<()>;
pub type I2 = I<I1>;
pub type I3 = I<I2>;
pub type I4 = I<I3>;

fn operate<D1, D2>(a: D1, b: D2) -> O<D1, D2>
where D1: DimensionType, D2: DimensionType,
{
    O(a, b)
}

/* impl<S1, I1, S2, I2, T> std::ops::Mul<(S2, I2, T)> for (S1, I1, T)
where S1: SignType, I1: NotZeroIdentityType,
S2: SignType, I2: NotZeroIdentityType,
{

} */

fn test() {
    let a = I(I(I(())));
    let a = I(());
    let a = BaseUnit(Positive, I(()), ());
    let b = BaseUnit(Positive, I(I(())), ());
    let r = O(a, b).unify();

    let a = BaseUnit(Positive, I(()), ());
    let b = BaseUnit(Negative, I(I(())), ());
    let r = O(a, b).unify();

    let a = BaseUnit(Positive, I(()), ());
    let b = BaseUnit(Positive, I(I(())), ()).inverse();
    let r = O(a, b).unify();

    let a = BaseUnit(Positive, I(I(())), ());
    let b = BaseUnit(Negative, I(I(())), ());
    let r = O(a, b).unify();

    let r = operate(a, b).unify();
}
