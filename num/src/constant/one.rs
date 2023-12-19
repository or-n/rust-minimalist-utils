pub trait One {
    fn one() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl One for $t {
            #[inline]
            fn one() -> Self {
                1 as Self
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);