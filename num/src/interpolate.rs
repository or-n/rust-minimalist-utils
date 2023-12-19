use crate::{operation::complement::*, scale::*};
use std::ops::{Add, Mul};

pub trait Interpolate<T, const N: usize> {
    fn interpolate(self, data: &[T; N]) -> T;
}

impl<T, Factor> Interpolate<T, 2> for Factor
where
    T: Copy + Add<Output = T> + Scale<Factor>,
    Factor: Copy + Complement,
{
    fn interpolate(self, data: &[T; 2]) -> T {
        data[0].scale(self.complement()) + data[1].scale(self)
    }
}

impl<T, Factor> Interpolate<T, 4> for [Factor; 2]
where
    T: Copy + Add<Output = T> + Scale<Factor>,
    Factor: Copy + Complement + Mul<Output = Factor>,
{
    #[inline]
    fn interpolate(self, data: &[T; 4]) -> T {
        let [no0, no1] = self.map(Complement::complement);
        let [yes0, yes1] = self;
        data[0].scale(no0 * no1)
            + data[1].scale(no0 * yes1)
            + data[2].scale(yes0 * no1)
            + data[3].scale(yes0 * yes1)
    }
}
