
pub trait SquareRoot {
    fn square_root(self) -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => {$(
        impl SquareRoot for $t {
            fn square_root(self) -> $t {
                self.sqrt()
            }
        }
    )*};
}

impl_for!(f32, f64);
