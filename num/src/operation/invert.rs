pub trait Invert {
    fn invert(self) -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl Invert for $t {
            #[inline]
            fn invert(self) -> $t {
                1 as Self / self
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
