use std::marker::PhantomData;

use super::super::math::{Float, Vec3, Basis, Origin, Position, Mat3};
use super::Mass;

pub trait SystemParticle<B, O> where B: Basis, O: Origin{
    fn mass(&self) -> Mass<Float>;
    fn relative_location(&self) -> Vec3<Position<B, O>>; // position_vector
    fn absolute_location(&self) -> Vec3<Position>;
}

/* pub struct SystemOfParticles<P, B, O>
where P: SystemParticle<B, O>, B: Basis, O: Origin
{
    particles: Vec<P>,
    basis: PhantomData<B>,
    origin: PhantomData<O>,
}
 */
/* pub trait SystemOfParticles<P, B, O, PB = B, PO = O> {

} */

pub trait SystemOfParticles<P, B, O>
where P: SystemParticle<B, O>, B: Basis, O: Origin
{
    fn relative_mass_center(&self) -> Vec3<Position<B, O>> {
        todo!()
    }
    fn absolute_mass_center(&self) -> Vec3<Position> {
        todo!()
    }
}

pub struct SystemOfMassCenter {}



