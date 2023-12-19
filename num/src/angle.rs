pub trait FromAngle {
    fn from_angle(self) -> (Self, Self)
    where
        Self: Sized;
}

pub trait ToAngle {
    fn to_angle(x: Self, y: Self) -> Self;
}

macro_rules! impl_for {
    ($($t:ty),*) => { $(
        impl FromAngle for $t {
            fn from_angle(self) -> (Self, Self) {
                (self.cos(), self.sin())
            }
        }

        impl ToAngle for $t {
            fn to_angle(x: Self, y: Self) -> Self {
                Self::atan2(y, x)
            }
        }
    )*};
}

impl_for!(f32, f64);
