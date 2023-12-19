pub mod f32;
pub mod epsilon;
pub mod rem_euclid;
pub mod max;

use crate::{scale::*, operation::invert::*};

pub fn ratio<T1: Into<F>, T2: Into<F>, F: Invert + Scale<F>>(
    index: T1, size: T2
) -> F {
    index.into().scale(size.into().invert())
}