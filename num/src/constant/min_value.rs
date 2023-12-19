pub trait MinValue {
    fn min_value() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl MinValue for $t {
            fn min_value() -> $t {
                <$t>::MIN
            }
        }
    )*};
}

impl_for!(usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
