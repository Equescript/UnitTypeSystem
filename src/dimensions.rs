/// Positive One
#[derive(Clone, Copy)]
pub struct P {}

/// Negative One
#[derive(Clone, Copy)]
pub struct N {}

/// Zero
#[derive(Clone, Copy)]
pub struct Z {}

#[derive(Clone, Copy)]
pub struct O<T, U>(pub T, pub U);

pub trait A {

}