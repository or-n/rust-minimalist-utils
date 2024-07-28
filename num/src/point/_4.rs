use crate::{
    constant::{max_value::*, min_value::*, one::*, zero::*},
    point::bounds::*,
    scale::*,
};
use array::fold_nonempty::*;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct _4<T>(pub [T; 4]);

impl<T: Copy> _4<T> {
    pub fn axis0(bounds: Bounds<T>) -> Self {
        let z = bounds.default;
        Self([bounds.other, z, z, z])
    }

    pub fn axis1(bounds: Bounds<T>) -> Self {
        let z = bounds.default;
        Self([z, bounds.other, z, z])
    }

    pub fn axis2(bounds: Bounds<T>) -> Self {
        let z = bounds.default;
        Self([z, z, bounds.other, z])
    }

    pub fn axis3(bounds: Bounds<T>) -> Self {
        let z = bounds.default;
        Self([z, z, z, bounds.other])
    }

    pub fn all_axis(bounds: Bounds<T>) -> [Self; 4] {
        [
            Self::axis0(bounds),
            Self::axis1(bounds),
            Self::axis2(bounds),
            Self::axis3(bounds),
        ]
    }
}

impl<A> _4<A> {
    pub fn zip<B, C>(self, other: _4<B>, f: impl Fn(A, B) -> C) -> _4<C> {
        let [a0, a1, a2, a3] = self.0;
        let [b0, b1, b2, b3] = other.0;
        _4([f(a0, b0), f(a1, b1), f(a2, b2), f(a3, b3)])
    }

    pub fn zip_mut<B>(&mut self, other: _4<B>, f: impl Fn(&mut A, B)) {
        let [a0, a1, a2, a3] = &mut self.0;
        let [b0, b1, b2, b3] = other.0;
        f(a0, b0);
        f(a1, b1);
        f(a2, b2);
        f(a3, b3);
    }
}

zero_one_minvalue_maxvalue!(_4, 4);
add_sub_mul_div_neg_scale_foldnonempty!(_4);
deref!(_4, 4);
