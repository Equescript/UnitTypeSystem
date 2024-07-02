/// Positive One
#[derive(Clone, Copy)]
pub struct P {}

/// Negative One
#[derive(Clone, Copy)]
pub struct N {}

#[derive(Clone, Copy)]
pub struct Unitless {}

/// Operation between T and U
#[derive(Clone, Copy)]
pub struct O<T, U>(pub T, pub U);

pub trait IsStructO {

}

impl<T, U> IsStructO for O<T, U> {

}

pub trait Simplify {
    type Output;
    fn simplify(self) -> Self::Output;
    type Reverse;
    fn reverse(self) -> Self::Reverse;
}

impl Simplify for P {
    type Output = Self;
    fn simplify(self) -> Self::Output {
        self
    }
    type Reverse = N;
    fn reverse(self) -> Self::Reverse {
        N {}
    }
}

impl Simplify for N {
    type Output = Self;
    fn simplify(self) -> Self::Output {
        self
    }
    type Reverse = P;
    fn reverse(self) -> Self::Reverse {
        P {}
    }
}

impl Simplify for O<P, P> {
    type Output = Self;
    fn simplify(self) -> Self::Output {
        self
    }
    type Reverse = O<N, N>;
    fn reverse(self) -> Self::Reverse {
        O(N {}, N {})
    }
}

impl Simplify for O<N, N> {
    type Output = Self;
    fn simplify(self) -> Self::Output {
        self
    }
    type Reverse = O<P, P>;
    fn reverse(self) -> Self::Reverse {
        O(P {}, P {})
    }
}

impl Simplify for O<P, N> {
    type Output = Unitless;
    fn simplify(self) -> Self::Output {
        Unitless {}
    }
    type Reverse = Unitless;
    fn reverse(self) -> Self::Reverse {
        Unitless {}
    }
}

impl Simplify for O<N, P> {
    type Output = Unitless;
    fn simplify(self) -> Self::Output {
        Unitless {}
    }
    type Reverse = Unitless;
    fn reverse(self) -> Self::Reverse {
        Unitless {}
    }
}

impl Simplify for O<N, Unitless> {
    type Output = N;
    fn simplify(self) -> Self::Output {
        N {}
    }
    type Reverse = P;
    fn reverse(self) -> Self::Reverse {
        P {}
    }
}

impl Simplify for O<Unitless, N> {
    type Output = N;
    fn simplify(self) -> Self::Output {
        N {}
    }
    type Reverse = P;
    fn reverse(self) -> Self::Reverse {
        P {}
    }
}

impl Simplify for O<P, Unitless> {
    type Output = P;
    fn simplify(self) -> Self::Output {
        P {}
    }
    type Reverse = N;
    fn reverse(self) -> Self::Reverse {
        N {}
    }
}

impl Simplify for O<Unitless, P> {
    type Output = P;
    fn simplify(self) -> Self::Output {
        P {}
    }
    type Reverse = N;
    fn reverse(self) -> Self::Reverse {
        N {}
    }
}

impl Simplify for O<Unitless, Unitless> {
    type Output = Unitless;
    fn simplify(self) -> Self::Output {
        Unitless {}
    }
    type Reverse = Unitless;
    fn reverse(self) -> Self::Reverse {
        Unitless {}
    }
}

impl<T: Simplify+IsStructO> Simplify for O<T, Unitless> {
    type Output = <T as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(o, _) = self;
        <T as Simplify>::simplify(o)
    }
    type Reverse = <T as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(o, _) = self;
        <T as Simplify>::reverse(o)
    }
}

impl<T: Simplify+IsStructO> Simplify for O<Unitless, T> {
    type Output = <T as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(_, o) = self;
        <T as Simplify>::simplify(o)
    }
    type Reverse = <T as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(_, o) = self;
        <T as Simplify>::reverse(o)
    }
}

impl<T: Simplify> Simplify for O<O<T, P>, P>
where
    O<T, P>: Simplify
{
    type Output = O<<O<T, P> as Simplify>::Output, P>;
    fn simplify(self) -> Self::Output {
        let Self(o, _) = self;
        O(<O<T, P> as Simplify>::simplify(o), P {})
    }
    type Reverse = O<<O<T, P> as Simplify>::Reverse, N>;
    fn reverse(self) -> Self::Reverse {
        let Self(o, _) = self;
        O(<O<T, P> as Simplify>::reverse(o), N {})
    }
}

impl<T: Simplify> Simplify for O<O<T, N>, N>
where
    O<T, N>: Simplify
{
    type Output = O<<O<T, N> as Simplify>::Output, N>;
    fn simplify(self) -> Self::Output {
        let Self(o, _) = self;
        O(<O<T, N> as Simplify>::simplify(o), N {})
    }
    type Reverse = O<<O<T, N> as Simplify>::Reverse, P>;
    fn reverse(self) -> Self::Reverse {
        let Self(o, _) = self;
        O(<O<T, N> as Simplify>::reverse(o), P {})
    }
}

impl<T: Simplify> Simplify for O<O<T, P>, N> {
    type Output = <T as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(O(o, _), _) = self;
        <T as Simplify>::simplify(o)
    }
    type Reverse = <T as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(O(o, _), _) = self;
        <T as Simplify>::reverse(o)
    }
}

impl<T: Simplify> Simplify for O<O<T, N>, P> {
    type Output = <T as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(O(o, _), _) = self;
        <T as Simplify>::simplify(o)
    }
    type Reverse = <T as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(O(o, _), _) = self;
        <T as Simplify>::reverse(o)
    }
}

impl<T: Simplify> Simplify for O<P, O<T, P>>
where
    O<T, P>: Simplify
{
    type Output = <O<O<T, P>, P> as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(_, o) = self;
        <O<O<T, P>, P> as Simplify>::simplify(O(o, P {}))
    }
    type Reverse = <O<O<T, P>, P> as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(_, o) = self;
        <O<O<T, P>, P> as Simplify>::reverse(O(o, P {}))
    }
}

impl<T: Simplify> Simplify for O<N, O<T, N>>
where
    O<T, N>: Simplify
{
    type Output = <O<O<T, N>, N> as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(_, o) = self;
        <O<O<T, N>, N> as Simplify>::simplify(O(o, N {}))
    }
    type Reverse = <O<O<T, N>, N> as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(_, o) = self;
        <O<O<T, N>, N> as Simplify>::reverse(O(o, N {}))
    }
}

impl<T: Simplify> Simplify for O<P, O<T, N>> {
    type Output = <T as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(_, O(o, _)) = self;
        <T as Simplify>::simplify(o)
    }
    type Reverse = <T as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(_, O(o, _)) = self;
        <T as Simplify>::reverse(o)
    }
}

impl<T: Simplify> Simplify for O<N, O<T, P>> {
    type Output = <T as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(_, O(o, _)) = self;
        <T as Simplify>::simplify(o)
    }
    type Reverse = <T as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(_, O(o, _)) = self;
        <T as Simplify>::reverse(o)
    }
}

impl<T: Simplify, U: Simplify> Simplify for O<O<T, P>, O<U, P>>
where
    O<O<O<T, P>, P>, U>: Simplify
{
    type Output = <O<O<O<T, P>, P>, U> as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(t, O(u, _)) = self;
        <O<O<O<T, P>, P>, U> as Simplify>::simplify(O(O(t, P {}), u))
    }
    type Reverse = <O<O<O<T, P>, P>, U> as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(t, O(u, _)) = self;
        <O<O<O<T, P>, P>, U> as Simplify>::reverse(O(O(t, P {}), u))
    }
}

impl<T: Simplify, U: Simplify> Simplify for O<O<T, N>, O<U, N>>
where
    O<O<O<T, N>, N>, U>: Simplify
{
    type Output = <O<O<O<T, N>, N>, U> as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(t, O(u, _)) = self;
        <O<O<O<T, N>, N>, U> as Simplify>::simplify(O(O(t, N {}), u))
    }
    type Reverse = <O<O<O<T, N>, N>, U> as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(t, O(u, _)) = self;
        <O<O<O<T, N>, N>, U> as Simplify>::reverse(O(O(t, N {}), u))
    }
}

impl<T: Simplify, U: Simplify> Simplify for O<O<T, P>, O<U, N>>
where
    O<T, U>: Simplify
{
    type Output = <O<T, U> as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(O(t, _), O(u, _)) = self;
        let a = O(t, u);
        <O<T, U> as Simplify>::simplify(a)
    }
    type Reverse = <O<T, U> as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(O(t, _), O(u, _)) = self;
        let a = O(t, u);
        <O<T, U> as Simplify>::reverse(a)
    }
}

impl<T: Simplify, U: Simplify> Simplify for O<O<T, N>, O<U, P>>
where
    O<T, U>: Simplify
{
    type Output = <O<T, U> as Simplify>::Output;
    fn simplify(self) -> Self::Output {
        let Self(O(t, _), O(u, _)) = self;
        let a = O(t, u);
        <O<T, U> as Simplify>::simplify(a)
    }
    type Reverse = <O<T, U> as Simplify>::Reverse;
    fn reverse(self) -> Self::Reverse {
        let Self(O(t, _), O(u, _)) = self;
        let a = O(t, u);
        <O<T, U> as Simplify>::reverse(a)
    }
}

pub trait Dimension: Copy {
    type UnifyOutput;
    fn unify(self) -> Self::UnifyOutput;
    type Inverse;
    fn inverse(self) -> Self::Inverse;
    // fn value() -> Self;
    const Value: Self;
}

impl Dimension for P {
    type UnifyOutput = Self;
    fn unify(self) -> Self::UnifyOutput {
        self
    }
    type Inverse = N;
    fn inverse(self) -> Self::Inverse {
        N {}
    }
    const Value: Self = P {};
}

impl Dimension for N {
    type UnifyOutput = Self;
    fn unify(self) -> Self::UnifyOutput {
        self
    }
    type Inverse = P;
    fn inverse(self) -> Self::Inverse {
        P {}
    }
    const Value: Self = N {};
}

impl Dimension for Unitless {
    type UnifyOutput = Self;
    fn unify(self) -> Self::UnifyOutput {
        self
    }
    type Inverse = Self;
    fn inverse(self) -> Self::Inverse {
        self
    }
    const Value: Self = Unitless {};
}

impl<T: Dimension, U: Dimension> Dimension for O<T, U>
where
    O<T, U>: Simplify
{
    type UnifyOutput = <O<T, U> as Simplify>::Output;
    fn unify(self) -> Self::UnifyOutput {
        self.simplify()
    }
    type Inverse = <O<T, U> as Simplify>::Reverse;
    fn inverse(self) -> Self::Inverse {
        self.reverse()
    }
    const Value: Self = O(T::Value, U::Value);
}

/* impl Unitless {
    pub const Value: Self = Self {};
} */

// impl P { pub const Value: Self = P {}; }
pub type P2 = O<P, P>; // impl P2 { pub const Value: Self = O(P {}, P {}); }
pub type P3 = O<P2, P>; // impl P3 { pub const Value: Self = O(P2::Value, P {}); }
pub type P4 = O<P3, P>; // impl P4 { pub const Value: Self = O(P3::Value, P {}); }
pub type P5 = O<P4, P>; // impl P5 { pub const Value: Self = O(P4::Value, P {}); }
pub type P6 = O<P5, P>; // impl P6 { pub const Value: Self = O(P5::Value, P {}); }
pub type P7 = O<P6, P>; // impl P7 { pub const Value: Self = O(P6::Value, P {}); }
pub type P8 = O<P7, P>; // impl P8 { pub const Value: Self = O(P7::Value, P {}); }

// impl N { pub const Value: Self = N {}; }
pub type N2 = O<N, N>; // impl N2 { pub const Value: Self = O(N {}, N {}); }
pub type N3 = O<N2, N>; // impl N3 { pub const Value: Self = O(N2::Value, N {}); }
pub type N4 = O<N3, N>; // impl N4 { pub const Value: Self = O(N3::Value, N {}); }
pub type N5 = O<N4, N>; // impl N5 { pub const Value: Self = O(N4::Value, N {}); }
pub type N6 = O<N5, N>; // impl N6 { pub const Value: Self = O(N5::Value, N {}); }
pub type N7 = O<N6, N>; // impl N7 { pub const Value: Self = O(N6::Value, N {}); }
pub type N8 = O<N7, N>; // impl N8 { pub const Value: Self = O(N7::Value, N {}); }

fn test() {
    let a: N2 = P2::Value.inverse();
    let b: P5 = N5::Value.inverse();
    let ab = O(a, b).simplify();
    // let a = P:
}

/* pub mod consts {
    pub const UNITLESS: super::Unitless = super::Unitless::Value;

    pub const P: super::P = super::P::Value;
    pub const P2: super::P2 = super::P2::Value;
    pub const P3: super::P3 = super::P3::Value;
    pub const P4: super::P4 = super::P4::Value;
    pub const P5: super::P5 = super::P5::Value;
    pub const P6: super::P6 = super::P6::Value;
    pub const P7: super::P7 = super::P7::Value;
    pub const P8: super::P8 = super::P8::Value;

    pub const N: super::N = super::N::Value;
    pub const N2: super::N2 = super::N2::Value;
    pub const N3: super::N3 = super::N3::Value;
    pub const N4: super::N4 = super::N4::Value;
    pub const N5: super::N5 = super::N5::Value;
    pub const N6: super::N6 = super::N6::Value;
    pub const N7: super::N7 = super::N7::Value;
    pub const N8: super::N8 = super::N8::Value;
} */