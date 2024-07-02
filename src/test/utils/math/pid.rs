use std::marker::PhantomData;

use super::float_point::Float;


pub trait CalculateCorrection<T, E = ()> {
    fn correction(&self, err: T) -> T;
    fn checked_correction(&self, err: T) -> Result<T, E> {
        Ok(self.correction(err))
    }
    /* fn add_correction(&self, err: T, value: T) -> T {
        value + self.correction(err)
    } */
}

pub trait CalculateCorrectionAppend<T, E = ()>: CalculateCorrection<T, E> where T: std::ops::Add<Output = T> {
    fn correction_append(&self, value: T, err: T) -> T {
        value + self.correction(err)
    }
    fn checked_correction_append(&self, value: Result<T, E>, err: T) -> Result<T, E> {
        Ok(value? + self.checked_correction(err)?)
    }
}

impl<T> CalculateCorrection<T> for () {
    fn correction(&self, err: T) -> T {
        err
    }
}

impl<T> CalculateCorrectionAppend<T> for () where T: std::ops::Add<Output = T> {
    fn correction_append(&self, value: T, _: T) -> T {
        value
    }
}

pub struct PID<T, P, I = (), D = ()> where
T: std::ops::Add<Output = T> + Clone,
P: CalculateCorrection<T>,
I: CalculateCorrectionAppend<T>,
D: CalculateCorrectionAppend<T>,
{
    p: P,
    i: I,
    d: D,
    t: PhantomData<T>,
}

impl<T, P, I, D> CalculateCorrection<T> for PID<T, P, I, D> where
T: std::ops::Add<Output = T> + Clone,
P: CalculateCorrection<T>,
I: CalculateCorrectionAppend<T>,
D: CalculateCorrectionAppend<T>,
{
    fn correction(&self, err: T) -> T {
        self.d.correction_append(
            self.i.correction_append(
                self.p.correction(err.clone()),
                err.clone()
            ),
            err
        )
    }
}
