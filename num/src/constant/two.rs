pub trait Two {
    fn two() -> Self;
}

impl Two for f32 {
    fn two() -> Self {
        2.0
    }
}