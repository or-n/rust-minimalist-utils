pub trait Epsilon {
    fn epsilon() -> Self;
}

impl Epsilon for f32 {
    fn epsilon() -> Self {
        f32::EPSILON
    }
}