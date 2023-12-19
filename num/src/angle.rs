pub trait Angle {
    fn angle(x: Self, y: Self) -> Self;
}

impl Angle for f32 {
    fn angle(x: Self, y: Self) -> Self {
        f32::atan2(x, y)
    }
}

pub trait FromAngle where Self: Sized {
    fn from_angle(self) -> (Self, Self);
}

impl FromAngle for f32 {
    fn from_angle(self) -> (Self, Self) {
        self.sin_cos()
    }
}