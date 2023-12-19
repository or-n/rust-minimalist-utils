pub trait Four {
    fn four() -> Self;
}

impl Four for f32 {
    fn four() -> Self {
        4.0
    }
}