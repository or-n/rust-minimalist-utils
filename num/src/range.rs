use crate::operation::complement::*;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

pub trait Mix<T, R> {
    fn mix(&self, ratio: R) -> T;
}

impl<T, R> Mix<T, R> for Range<T>
where
    T: Copy + Mul<R, Output = T> + Add<Output = T>,
    R: Copy + Complement,
{
    fn mix(&self, ratio: R) -> T {
        self.start * ratio.complement() + self.end * ratio
    }
}

pub trait Ratio<T, R> {
    fn ratio(&self, mix: T) -> R;
}

impl<T, R> Ratio<T, R> for Range<T>
where
    T: Copy + Sub<Output = T> + Div<Output = R>,
{
    fn ratio(&self, mix: T) -> R {
        (mix - self.start) / (self.end - self.start)
    }
}

pub trait Map<T, R> {
    fn map(&self, other: &Self, mix: T) -> T;
}

impl<T, R> Map<T, R> for Range<T>
where
    Range<T>: Ratio<T, R> + Mix<T, R>,
{
    fn map(&self, other: &Self, mix: T) -> T {
        other.mix(self.ratio(mix))
    }
}

/*
impl<T: Copy> Range<T> {
    pub fn mix(&self, ratio: f32) -> T
    where
        T: Mul<f32, Output = T> + Add<Output = T>,
    {
        self.start * (1.0 - ratio) + self.end * ratio
    }

    pub fn ratio(&self, mix: T) -> f32
    where
        T: Sub<Output = T> + Div<Output = f32>,
    {
        (mix - self.start) / (self.end - self.start)
    }

    pub fn map<'a>(&'a self, other: &'a Self) -> impl Fn(T) -> T + 'a
    where
        T: Mul<f32, Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = f32>,
    {
        move |mix| other.mix(self.ratio(mix))
    }
}*/
