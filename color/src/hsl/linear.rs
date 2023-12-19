use num::{
    point::{_2::*, _3::*},
    scale::*,
    angle::*
};

use crate::hsl::*;

#[derive(Clone, Copy, Debug)]
pub struct LinearHSL<T>(pub (_2<T>, _2<T>));

impl<T: Copy + FromAngle> Into<LinearHSL<T>> for HSL<T> {
    fn into(self) -> LinearHSL<T> {
        let [h, s, l] = self.0.0;
        let xy = h.from_angle();
        LinearHSL((_2(xy.into()), _2([s, l])))
    }
}

impl<T: Angle> Into<HSL<T>> for LinearHSL<T> {
    fn into(self) -> HSL<T> {
        let (xy, sl) = self.0;
        let [x, y] = xy.0;
        let [s, l] = sl.0;
        let h = T::angle(x, y);
        HSL(_3([h, s, l]))
    }
}

impl<T> Add for LinearHSL<T>
where
    T: Add<Output = T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (xy1, sl1) = self.0;
        let (xy2, sl2) = other.0;
        Self((xy1 + xy2, sl1 + sl2))
    }
}

impl<T> Scale<f32> for LinearHSL<T>
where
    T: Scale<f32>
{
    fn scale(self, factor: f32) -> Self {
        let (xy, sl) = self.0;
        Self((xy.scale(factor), sl.scale(factor)))
    }
}