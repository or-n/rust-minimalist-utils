pub trait Five {
    fn five() -> Self;
}

impl Five for f32 {
    fn five() -> Self {
        5.0
    }
}