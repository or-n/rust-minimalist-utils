pub trait RemEuclid {
    fn rem_euclid(self, other: Self) -> Self;
}

impl RemEuclid for f32 {
    fn rem_euclid(self, other: Self) -> Self {
        self.rem_euclid(other)
    }
}