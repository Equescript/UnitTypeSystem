pub mod base_units;
pub mod dimension;

use dimension::BaseUnit;

pub use dimension::{Unitless, I1, I2, I3, I4};
pub use dimension::Positive as P;
pub use dimension::Negative as N;
pub use base_units::{Unit, UnitOperation, Unify};

pub type Second<S, I> = BaseUnit<S, I, base_units::Second>;
pub type Frame<S, I> = BaseUnit<S, I, base_units::Frame>;
pub type Meter<S, I> = BaseUnit<S, I, base_units::Meter>;
pub type Kg<S, I> = BaseUnit<S, I, base_units::Kg>;
pub type Radian<S, I> = BaseUnit<S, I, base_units::Radian>;
pub type Degree<S, I> = BaseUnit<S, I, base_units::Degree>;

