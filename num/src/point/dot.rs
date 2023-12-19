use array::fold::*;
use std::ops::{Add, Mul};

pub trait Dot<Number> {
    fn dot(self, other: Self) -> Number;
}

impl<Point, T> Dot<T> for Point
where
    Point: Mul<Output = Point> + FoldNonempty<T>,
    T: Add<Output = T>
{
    fn dot(self, other: Self) -> T {
        (self * other).fold_nonempty(Add::add)
    }
}