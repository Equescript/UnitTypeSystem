use std::ops::{Add, Sub, Neg, Mul};
pub trait Group: Sized + Add<Output=Self> + Sub<Output=Self> + Neg<Output=Self> {
    // const IDENTITY: Self;
}

pub trait AbelianGroup: Group {

}

pub trait Ring: AbelianGroup + Mul<Output=Self> {

}

pub trait CommutativeRing: Ring {

}

/*

location + velocity * time

 */
