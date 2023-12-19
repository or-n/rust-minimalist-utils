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
/*

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl Decrement for $t {
            #[inline]
            fn decrement(self) -> $t {
                self - <$t>::one()
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
*/
