use crate::constant::{min_value::*, max_value::*};

struct ExactF32Integer<T>(T);

macro_rules! impl_min_for {
    ($($t:ty),*) => {$(
        impl MinValue for ExactF32Integer<$t> {
            fn min_value() -> Self {
                ExactF32Integer(1 - 2^24)
            }
        }
    )*};
}

macro_rules! impl_max_for {
    ($($t:ty),*) => {$(
        impl MaxValue for ExactF32Integer<$t> {
            fn max_value() -> Self {
                ExactF32Integer(2^24 - 1)
            }
        }
    )*};
}

impl_min_for!(isize, i32, i64);
impl_max_for!(isize, i32, i64, usize, u32, u64);

pub trait UnsafeIntoF32 {
    fn to_f32(self) -> f32;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl UnsafeIntoF32 for $t {
            fn to_f32(self) -> f32 {
                self as f32
            }
        }
    )*};
}

impl_for!(isize, i32, i64, usize, u32, u64);

pub fn f32_ratio<T1: UnsafeIntoF32, T2: UnsafeIntoF32>(index: T1, size: T2) -> f32 {
    index.to_f32() / size.to_f32()
}
