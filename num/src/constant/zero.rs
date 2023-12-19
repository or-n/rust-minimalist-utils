pub trait Zero {
    fn zero() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl Zero for $t {
            fn zero() -> Self {
                0 as Self
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
