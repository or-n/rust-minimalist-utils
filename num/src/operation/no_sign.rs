pub trait NoSign {
    fn no_sign(self) -> Self;
}

impl NoSign for f32 {
    #[inline]
    fn no_sign(self) -> Self {
        self.abs()
    }
}