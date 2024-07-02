use super::super::math::{Float, Vec3, Basis, Origin, Position, Direction, Mat3};
// use super::units::Unitless;
use super::{Unify, Unitless, Time, Mass, Inertia, Location, Velocity, Accel, AngularVelocity};

/* pub trait DynamicStatus<B: Basis, O: Origin> {
    fn frame_index(&self) -> usize;
    fn frame_time(&self) -> Time<Float>;
    fn mass(&self) -> Mass<Float>;
    // / rotational inertia 转动惯量
    fn inertia(&self) -> Inertia<Float>;
    fn location(&self) -> Location<Vec3<Position>> {
        O::instance().location().into()
    }
    fn basis_matrix(&self) -> Mat3 {
        B::instance().transformation()
    }
} */

#[derive(Clone, Copy)]
pub struct DynamicStatus<B = (), O = ()> where B: Basis, O: Origin {
    location: Location<Vec3<Position<B, O>>>,
    velocity: Velocity<Vec3<Direction<B>>>,
    // angular_velocity: AngularVelocity<Vec3<Direction<B>>>,
    // basis: Mat3,
}

#[derive(Clone, Copy)]
pub struct DynamicStatusSlice<B = (), O = ()> where B: Basis, O: Origin {
    start: DynamicStatus<B, O>,
    end: DynamicStatus<B, O>,
    accel: Accel<Vec3<Direction<B>>>,
    time: Time<Float>,
}

impl<B, O> DynamicStatusSlice<B, O> where B: Basis, O: Origin {
    fn next(&self, next_accel: Accel<Vec3<Direction<B>>>, time: Time<Float>) -> Self {
        let delta_v: Velocity<Vec3<Direction<B>>> = (next_accel * time).unify();
        Self {
            start: self.end,
            end: DynamicStatus {
                location: self.end.location + ((self.end.velocity + delta_v * Float::from(0.5)) * time).unify(),
                velocity: self.end.velocity + delta_v,
            },
            accel: next_accel,
            time,
        }
    }
}

#[derive(Clone)]
pub struct DynamicStatusSequence<B = (), O = ()> where B: Basis, O: Origin {
    sequence: Vec<DynamicStatusSlice<B, O>>
}

