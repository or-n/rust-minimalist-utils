pub trait Gen {
    fn generate() -> Self;
}

pub trait Coords<P> {
    fn coords(&self) -> Vec<P>;
}

pub trait ToIndex {
    fn index(self) -> usize;
}

pub trait Dot {
    fn dot(self, other: Self) -> Self;
}
