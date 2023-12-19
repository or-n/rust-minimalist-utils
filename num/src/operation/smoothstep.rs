use crate::constant::{two::*, three::*};
use std::ops::{Mul, Sub};

pub fn smoothstep<T>(x: T) -> T
where
    T: Two + Three
        + Mul<Output = T>
        + Sub<Output = T>
        + Copy
{
    x * x * (T::three() - T::two() * x)
}