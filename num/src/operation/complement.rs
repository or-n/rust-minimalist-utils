use crate::constant::{max_value::*, one::*};
use std::ops::Sub;

pub trait Complement {
    fn complement(self) -> Self;
}

impl<T: One + Sub<Output = T>> Complement for T {
    #[inline]
    fn complement(self) -> Self {
        Self::one() - self
    }
}

pub trait ComplementMax {
    fn complement_max(self) -> Self;
}

impl<T: MaxValue + Sub<Output = T>> ComplementMax for T {
    #[inline]
    fn complement_max(self) -> Self {
        Self::max_value() - self
    }
}
