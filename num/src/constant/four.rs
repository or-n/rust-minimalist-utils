pub trait Four {
    fn four() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl Four for $t {
            #[inline]
            fn four() -> Self {
                4 as Self
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
