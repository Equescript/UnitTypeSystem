mod test;
mod unit;
mod units;
mod physical_quantity;
mod system;
mod status;

use units::{Unit, P, N, I1, I2, Second, Frame, Meter, Kg, Radian};
use units::Unitless as U;
pub use units::Unify;
pub use physical_quantity::{UnitlessValue, PQ};

pub type Unitless<T> =          PQ<T, Unit<U,             U,            U,         U>>;
pub type Time<T> =              PQ<T, Unit<Second<P, I1>, U,            U,         U>>;
// pub type TimeF<T> =             PQ<T, Unit<Frame <P, I1>, U,            U,         U>>;
pub type Mass<T> =              PQ<T, Unit<U,             U,            Kg<P, I1>, U>>;
pub type Inertia<T> =           PQ<T, Unit<U,             Meter<N, I2>, Kg<P, I1>, U>>;
pub type Length<T> =            PQ<T, Unit<U,             Meter<P, I1>, U,         U>>;
pub type Location<T> =          PQ<T, Unit<U,             Meter<P, I1>, U,         U>>;
pub type Velocity<T> =          PQ<T, Unit<Second<N, I1>, Meter<P, I1>, U,         U>>;
pub type AngularVelocity<T> =   PQ<T, Unit<Second<N, I1>, U,            U,         Radian<P, I1>>>;
// pub type VelocityF<T> =         PQ<T, Unit<Frame <N, I1>, Meter<P, I1>, U,         U>>;
pub type Accel<T> =             PQ<T, Unit<Second<N, I2>, Meter<P, I1>, U,         U>>;
// pub type AccelF<T> =            PQ<T, Unit<Frame <N, I2>, Meter<P, I1>, U,         U>>;
pub type Force<T> =             PQ<T, Unit<Second<N, I2>, Meter<P, I1>, Kg<P, I1>, U>>;
// pub type ForceF<T> =            PQ<T, Unit<Frame <N, I2>, Meter<P, I1>, Kg<P, I1>, U>>;

fn test() {

}
