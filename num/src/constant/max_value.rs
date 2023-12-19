pub trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl MaxValue for $t {
            fn max_value() -> $t {
                <$t>::MAX
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
