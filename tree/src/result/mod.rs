use crate::binary_operation::*;
use std::ops::{AddAssign, Sub};

#[derive(Debug, PartialEq)]
pub struct AddResult<T>(pub T);

impl Default for AddResult<i32> {
    fn default() -> Self {
        Self(0)
    }
}

impl<T> BinaryOperation<AddResult<T>> for T
where
    T: AddAssign<T>,
{
    fn operate(self, value: &mut AddResult<T>) {
        value.0 += self;
    }
}

#[derive(Debug, PartialEq)]
pub struct SubResult<T>(pub T);

impl Default for SubResult<i32> {
    fn default() -> Self {
        Self(0)
    }
}

impl<T> BinaryOperation<SubResult<T>> for T
where
    T: Copy + Sub<Output = T>,
{
    fn operate(self, value: &mut SubResult<T>) {
        value.0 = self - value.0;
    }
}
