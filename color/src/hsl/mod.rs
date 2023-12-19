pub mod linear;

use num::{
    constant::{zero::*, one::*, two::*, four::*, six::*},
    point::_3::*,
};
use std::ops::{Add, Sub, Div};

use crate::rgb::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct HSL<T>(pub _3<T>);

impl<T> Into<HSL<T>> for RGB<T>
where
    T: Zero + One + Two + Four + Six
        + Add<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + PartialOrd
        + Copy
{
    fn into(self) -> HSL<T> {
        let [r, g, b] = self.0.0;
        let my_min = |a: T, b: T| if a < b { a } else { b };
        let my_max = |a: T, b: T| if a > b { a } else { b };
        let min = my_min(r, my_min(g, b));
        let max = my_max(r, my_max(g, b));
        let both = min + max;
        let l = both / T::two();
        let (h, s) = if max <= min {
            (T::zero(), T::zero())
        } else {
            let d = max - min;
            let h_ = if max == r {
                (g - b) / d + if g < b { T::six() } else { T::zero() }
            } else if max == g {
                (b - r) / d + T::two()
            } else {
                (r - g) / d + T::four()
            };
            let s_ = if both > T::one() { T::two() - both } else { both };
            (h_ / T::six(), d / s_)
        };
        HSL(_3([h, s, l]))
    }
}