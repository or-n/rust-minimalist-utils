use std::ops::Neg;
use crate::constant::{zero::*, one::*, max_value::*};

#[derive(Clone, Copy)]
pub struct Bounds<T> {
    pub default: T,
    pub other: T
}

impl<T: Zero + One> One for Bounds<T> {
    fn one() -> Self {
        Bounds { default: T::zero(), other: T::one() }
    }
}

impl<T: Zero + MaxValue> MaxValue for Bounds<T> {
    fn max_value() -> Self {
        Bounds { default: T::zero(), other: T::max_value() }
    }
}

impl<T> Neg for Bounds<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Bounds { default: self.other, other: self.default }
    }
}