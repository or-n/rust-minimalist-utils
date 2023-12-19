use crate::operation::{square_root::*, invert::*};
use crate::{scale::*, point::dot::*};

pub trait Length<Number> {
    fn length(&self) -> Number;
}

impl<Point: LengthSquared<N>, N: SquareRoot> Length<N> for Point {
    fn length(&self) -> N {
        self.length_squared().square_root()
    }
}

pub trait LengthSquared<Number> {
    fn length_squared(&self) -> Number;
}

impl<Point: Copy + Dot<N>, N> LengthSquared<N> for Point {
    fn length_squared(&self) -> N {
        self.dot(self.clone())
    }
}

pub trait Normalize<N> {
    fn normalize(self) -> Self;
}

impl<Point: Length<N> + Scale<N>, N: Invert> Normalize<N> for Point {
    fn normalize(self) -> Self {
        let length = self.length();
        self.scale(length.invert())
    }
}