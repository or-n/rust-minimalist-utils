use crate::constant::one::*;
use std::ops::Sub;

pub trait Decrement {
    fn decrement(self) -> Self;
}

impl<T: One + Sub<Output = T>> Decrement for T {
    fn decrement(self) -> Self {
        self - Self::one()
    }
}
