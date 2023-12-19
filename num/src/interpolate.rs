use crate::{operation::complement::*, scale::*};
use std::ops::{Add, Mul};

pub trait Interpolate<T, const N: usize> {
    fn interpolate(self, data: &[T; N]) -> T;
}

impl<T, Factor> Interpolate<T, 2> for Factor
where
    T: Copy + Add<Output = T> + Scale<Factor>,
    Factor: Copy + Complement
{
    fn interpolate(self, data: &[T; 2]) -> T {
        data[0].scale(self.complement()) +
        data[1].scale(self)
    }
}

impl<T, Factor> Interpolate<T, 4> for [Factor; 2]
where
    T: Copy + Add<Output = T> + Scale<Factor>,
    Factor: Copy + Complement + Mul<Output = Factor>
{
    #[inline]
    fn interpolate(self, data: &[T; 4]) -> T {
        let [no0, no1] = self.map(Complement::complement);
        let [yes0, yes1] = self;  
        data[0].scale(no0 * no1) +
        data[1].scale(no0 * yes1) +
        data[2].scale(yes0 * no1) +
        data[3].scale(yes0 * yes1)
    }
}

pub fn cyclic_interpolate(values: [f32; 1 << 1], t: f32) -> f32 {
    let mut start = values[0];
    let mut end = values[1];
    let diff = end - start;
    if diff.abs() > 0.5 {
        if end > start {
            start += 1.0;
        } else {
            end += 1.0;
        }
    }
    let v = start + (end - start) * t;
    v % 1.0
}

/*
#[inline]
pub fn interpolate2d<T, Factor>(values: [T; 1 << 2], ratio: _2<Factor>) -> T
where
    T: Copy,
    Factor: Copy + Interpolate<T>
    //T: Copy + Add<Output = T> + Scale<Factor>,
    //Factor: Copy + Mul<Output = Factor> + Complement,
    //_2<Factor>: Complement
{
    //let [no0, no1] = ratio.0.map(Complement::complement);
    let [yes0, yes1] = ratio.0;    

    let l = yes1.interpolate(values[0], values[1]);
    let r = yes1.interpolate(values[2], values[3]);
    let h = yes0.interpolate(l, r);

    let t = yes0.interpolate(values[0], values[2]);
    let b = yes0.interpolate(values[1], values[3]);
    let v = yes1.interpolate(t, b);

    //let x = yes0.interpolate(h, v);
    //let y = yes1.interpolate(h, v);

    //yes1.interpolate(x, y)

    h
/*    
    let t = yes0.interpolate(values[0], values[2]);
    let b = yes0.interpolate(values[1], values[3]);
    yes1.interpolate(t, b)

    values[0].scale(no0 * no1) +
    values[1].scale(no0 * yes1) +
    values[2].scale(yes0 * no1) +
    values[3].scale(yes0 * yes1)

    
  
    let duals = ratio.complement().zip(ratio, |no, yes| [no, yes]).0;
    let factors = mix(duals[0], duals[1], Mul::mul);
    fold(zip(values, factors, Scale::scale), Add::add)
    */
}

pub fn _interpolate3d<T, Factor>(values: [T; 1 << 3], ratio: _3<Factor>) -> T
where
    T: Copy + Add<Output = T> + Scale<Factor>,
    Factor: Copy + Mul<Output = Factor>,
    _3<Factor>: Complement
{
    let duals = zip(ratio.complement().0, ratio.0, |no, yes| [no, yes]);
    let factors: [Factor; 1 << 2] = mix(duals[0], duals[1], Mul::mul);
    let factors2 = mix(factors, duals[2], Mul::mul);
    fold(zip(values, factors2, Scale::scale), Add::add)
}

*/
