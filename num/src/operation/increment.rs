use crate::constant::one::*;
use std::ops::Add;

pub trait Increment {
    fn increment(self) -> Self;
}

impl<T: One + Add<Output = T>> Increment for T {
    fn increment(self) -> Self {
        self + Self::one()
    }
}
