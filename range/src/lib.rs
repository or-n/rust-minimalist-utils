use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct MinSize<T> {
    min: T,
    size: T,
}

#[derive(Debug, Clone, Copy)]
pub struct MinMax<T> {
    min: T,
    max: T,
}

impl<T> From<MinMax<T>> for MinSize<T>
where
    T: Copy + Sub<Output = T>,
{
    fn from(value: MinMax<T>) -> Self {
        Self {
            min: value.min,
            size: value.max - value.min,
        }
    }
}

impl<T> From<MinSize<T>> for MinMax<T>
where
    T: Copy + Add<Output = T>,
{
    fn from(value: MinSize<T>) -> Self {
        Self {
            min: value.min,
            max: value.min + value.size,
        }
    }
}
