pub trait Three {
    fn three() -> Self;
}

impl Three for f32 {
    fn three() -> Self {
        3.0
    }
}