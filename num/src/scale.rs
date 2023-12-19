pub trait Scale<T> {
    fn scale(self, factor: T) -> Self;
}

macro_rules! impl_for {
    ($for:ty, ($($t:ty),*)) => {$(
        impl Scale<$t> for $for {
            #[inline]
            fn scale(self, factor: $t) -> Self {
                (self as $t * factor) as Self
            }
        }
    )*};
}

impl_for!(usize, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(isize, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(u8, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(u16, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(u32, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(u64, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(i8, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(i16, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(i32, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(i64, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(f32, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
impl_for!(f64, (usize, isize, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64));
