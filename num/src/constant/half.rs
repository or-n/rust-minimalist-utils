pub trait Half {
    fn half() -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl Half for $t {
            #[inline]
            fn half() -> Self {
                0.5 as Self
            }
        }
    )*};
}

impl_for!(f32, f64);
