use num::{
    constant::{zero::*, one::*, two::*, three::*, four::*, five::*, six::*},
    operation::{decrement::*, complement::*, no_sign::*},
    point::_3::*,
};
use std::ops::{Add, Sub, Mul, Div, Rem};

use crate::hsl::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct RGB<T>(pub _3<T>);

impl<T> Into<RGB<T>> for HSL<T>
where
    T: Zero + One + Two + Three + Four + Five + Six
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + NoSign
        + PartialOrd
        + Copy
{
    fn into(self) -> RGB<T> {
        let [h, s, l] = self.0.0;
        let h = h * T::six();
        let modify = |x: T| x.decrement().no_sign().complement();
        let chroma = s * modify(l * T::two());
        let x = chroma * modify(h % T::two());
        let m = l - chroma / T::two();
        let (r, g, b) = if h < T::one() {
            (chroma, x, T::zero())
        } else if h < T::two() {
            (x, chroma, T::zero())
        } else if h < T::three() {
            (T::zero(), chroma, x)
        } else if h < T::four() {
            (T::zero(), x, chroma)
        } else if h < T::five() {
            (x, T::zero(), chroma)
        } else {
            (chroma, T::zero(), x)
        };
        RGB(_3([r + m, g + m, b + m]))
    }
}