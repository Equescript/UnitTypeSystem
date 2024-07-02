use std::ops::Sub;

pub trait ScalarField<D, R> where R: PartialOrd {
    fn field(&self, dom: D) -> R;
}

pub trait VectorField<D, R> {
    fn field(&self, dom: D) -> R;
}

pub struct Position<T>(T) where T: Sub<Output = T>;
pub struct Offset<T>(T) where T: Sub<Output = T>;

impl<T> Sub for Position<T> where T: Sub<Output = T> {
    type Output = Offset<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Offset(self.0 - rhs.0)
    }
}

