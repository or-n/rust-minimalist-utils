use crate::
    point::{_2::*, _3::*}
;

use std::ops::{Add, Sub, Mul, Div};

pub fn barycentric2<T, F>(triangle: _3<_2<T>>, factor: _2<T>) -> [F; 2]
where
    T: Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = F>
{
    let [a, b, c] = triangle.0;
    let factor = (factor - c).0;
    let bc = (b - c).0;
    let ca = (c - a).0;    
    let determinant = bc[0] * ca[1] - ca[0] * bc[1];
    let u1 = (bc[1] * factor[0] - bc[0] * factor[1]) / determinant;
    let u2 = (ca[1] * factor[0] - ca[0] * factor[1]) / determinant;
    [u1, u2]
}

pub fn barycentric3<T, F>(triangle: [_3<T>; 3], point: _3<T>) -> [F; 2]
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = F>,
{
    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    let v0 = (b - a).0;
    let v1 = (c - a).0;
    let v2 = (point - a).0;

    let dot00 = v0[0] * v0[0] + v0[1] * v0[1] + v0[2] * v0[2];
    let dot01 = v0[0] * v1[0] + v0[1] * v1[1] + v0[2] * v1[2];
    let dot02 = v0[0] * v2[0] + v0[1] * v2[1] + v0[2] * v2[2];
    let dot11 = v1[0] * v1[0] + v1[1] * v1[1] + v1[2] * v1[2];
    let dot12 = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];

    let denom = dot00 * dot11 - dot01 * dot01;

    let u1 = (dot11 * dot02 - dot01 * dot12) / denom;
    let u2 = (dot00 * dot12 - dot01 * dot02) / denom;
    [u1, u2]
}