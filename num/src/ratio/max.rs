use crate::{ratio::{*, f32::*}, constant::max_value::*};

pub fn clamp_max<T: UnsafeIntoF32>(x: T, max: T) -> f32 {
    f32_ratio(x, max).min(1.0)
}

pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

pub fn ratio_max<T: MaxValue + Into<f32>>(x: T) -> f32 {
    ratio(x, T::max_value())
}

pub fn unratio_max_u8(x: f32) -> u8 {
    x.scale(u8::max_value()) as u8 
}