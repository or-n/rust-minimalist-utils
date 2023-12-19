pub trait Five {
    fn five() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl Five for $t {
            #[inline]
            fn five() -> Self {
                5 as Self
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
