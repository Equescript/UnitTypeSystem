pub mod dimension;
pub mod dimensions;
pub mod physical_quantity;
pub mod si;
pub mod units;
mod test;

use dimension as dim;

use self::{units::SIUnit, physical_quantity::PhysicalQuantity};

type Unitless = SIUnit<dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// s
type Second = SIUnit<dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// m
type Meter = SIUnit<dim::Unitless, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// kg
type Kg = SIUnit<dim::Unitless, dim::Unitless, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// m/s
type MeterPerSecond = SIUnit<dim::N, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// m/s^2
type MeterPerSecondSquared = SIUnit<dim::N2, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// kg*m/s
type KgTimesMeterPerSecond = SIUnit<dim::N, dim::P, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// kg*m/s^2 = N
type Newton = SIUnit<dim::N2, dim::P, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// kg*m^2/s^2 = J
type Joule = SIUnit<dim::N2, dim::P2, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;

/// kg*m^2/s^3 = W
type Watt = SIUnit<dim::N3, dim::P2, dim::P, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless, dim::Unitless>;


fn main() {
    // Let's say we have a object with mass of 2.0 kg
    let mass: PhysicalQuantity<f64, Kg> = PhysicalQuantity(2.0, Kg::Value);
    // It's initial velocity is 1 m/s
    let initial_velocity: PhysicalQuantity<f64, MeterPerSecond> = PhysicalQuantity(1.0, MeterPerSecond::Value);
    // After 6 s
    let time: PhysicalQuantity<f64, Second> = PhysicalQuantity(6.0, Second::Value);
    // It's terminal velocity is 4 m/s
    let terminal_velocity: PhysicalQuantity<f64, MeterPerSecond> = PhysicalQuantity(4.0, MeterPerSecond::Value);

    // Calculate the accelation
    let accelation: PhysicalQuantity<f64, MeterPerSecondSquared> = ((terminal_velocity - initial_velocity) / time).unify();
    println!("Accelation: {}", accelation.0);
    // Output: Accelation: 0.5

    // Calculate the force, F = ma
    let force: PhysicalQuantity<f64, Newton> = (mass * accelation).unify();
    println!("Force: {}", force.0);
    // Output: Force: 1

    // Calculate the terminal kinetic energy E = 1/2 mv^2
    let kinetic_energy: PhysicalQuantity<f64, Joule> =  (((PhysicalQuantity(0.5, Unitless::Value) * mass).unify() * terminal_velocity).unify() * terminal_velocity).unify();
    println!("Kinetic Energy: {}", kinetic_energy.0);
    // Output: Kinetic Energy: 16
}
