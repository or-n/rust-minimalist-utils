use crate::{
    constant::{max_value::*, min_value::*, one::*, zero::*},
    point::{bounds::*, _4::*},
    scale::*,
};
use array::fold_nonempty::*;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct _3<T>(pub [T; 3]);

impl<T: Copy> _3<T> {
    pub fn axis0(bounds: Bounds<T>) -> Self {
        Self([bounds.other, bounds.default, bounds.default])
    }

    pub fn axis1(bounds: Bounds<T>) -> Self {
        Self([bounds.default, bounds.other, bounds.default])
    }

    pub fn axis2(bounds: Bounds<T>) -> Self {
        Self([bounds.default, bounds.default, bounds.other])
    }

    pub fn all_axis(bounds: Bounds<T>) -> [Self; 3] {
        [
            Self::axis0(bounds),
            Self::axis1(bounds),
            Self::axis2(bounds),
        ]
    }
}

impl<A> _3<A> {
    pub fn extend(self, value: A) -> _4<A> {
        let [a0, a1, a2] = self.0;
        _4([a0, a1, a2, value])
    }

    pub fn zip<B, C>(self, other: _3<B>, f: impl Fn(A, B) -> C) -> _3<C> {
        let [a0, a1, a2] = self.0;
        let [b0, b1, b2] = other.0;
        _3([f(a0, b0), f(a1, b1), f(a2, b2)])
    }

    pub fn zip_mut<B>(&mut self, other: _3<B>, f: impl Fn(&mut A, B)) {
        let [a0, a1, a2] = &mut self.0;
        let [b0, b1, b2] = other.0;
        f(a0, b0);
        f(a1, b1);
        f(a2, b2);
    }
}

zero_one_minvalue_maxvalue!(_3, 3);
add_sub_mul_div_neg_scale_foldnonempty!(_3);
deref!(_3, 3);
