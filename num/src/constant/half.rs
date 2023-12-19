pub trait Half {
    fn half() -> Self;
}

impl Half for f32 {
    fn half() -> Self {
        0.5
    }
}