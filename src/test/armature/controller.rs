use super::super::utils::math::{Float, Vec3, Direction, Position, Normalized, Mat3};
use super::super::utils::physics::{Time, Location, Velocity, Accel, Force};
pub trait Controller {
    // fn controller(&self, state: KinematicState);
}

// trajectory Move
// point

pub struct CenterBasis {}

pub enum BoundType {
    Location(Location<Vec3<Position>>),
    Velocity(Velocity<Vec3<Direction>>),
}

pub struct Target {
    // center_offset: Option<Location<Vec3<CenterBasis>>>,
    // center_basis_matrix: Option<Mat3>,
    bound: BoundType,
}

pub enum LegSupportState {
    Swing,
    Stance,
}

pub struct LegControlInfo {
    leg_support_state: LegSupportState,
    last_contact: Time<Float>,
    last_depart: Time<Float>,
    hoove_location: Location<Vec3<Position>>,
}

pub struct ContactPoint {
    location: Location<Vec3<Position>>,
    normal: Vec3<Direction<(), Normalized>>,
}

pub struct ContactForce {
    force: Force<Vec3<Direction>>,
}


// location