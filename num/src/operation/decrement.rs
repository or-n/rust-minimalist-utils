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
