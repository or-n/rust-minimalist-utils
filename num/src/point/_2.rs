use crate::{
    constant::{max_value::*, min_value::*, one::*, zero::*},
    point::{bounds::*, _3::*},
    scale::*,
};
use array::fold::*;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct _2<T>(pub [T; 2]);

impl<T: Copy> _2<T> {
    pub fn all_axis(bounds: Bounds<T>) -> [Self; 2] {
        [
            _2([bounds.other, bounds.default]),
            _2([bounds.default, bounds.other]),
        ]
    }
}

impl<A> _2<A> {
    pub fn extend(self, value: A) -> _3<A> {
        let [a0, a1] = self.0;
        _3([a0, a1, value])
    }

    pub fn zip<B, C>(self, other: _2<B>, f: impl Fn(A, B) -> C) -> _2<C> {
        let [a0, a1] = self.0;
        let [b0, b1] = other.0;
        _2([f(a0, b0), f(a1, b1)])
    }

    pub fn zip_mut<B>(&mut self, other: _2<B>, f: impl Fn(&mut A, B)) {
        let [a0, a1] = &mut self.0;
        let [b0, b1] = other.0;
        f(a0, b0);
        f(a1, b1);
    }
}

zero_one_minvalue_maxvalue!(_2, 2);
add_sub_mul_div_neg_scale_foldnonempty!(_2);

impl From<_2<u32>> for _2<i32> {
    fn from(item: _2<u32>) -> _2<i32> {
        _2(item.0.map(|x| x as i32))
    }
}
