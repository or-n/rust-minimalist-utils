
#[inline]
pub fn zip<X, Y, Z, const SIZE: usize>(
    xs: [X; SIZE],
    ys: [Y; SIZE],
    f: impl Fn(X, Y) -> Z
) -> [Z; SIZE] {
    array_operation!(SIZE, f, xs.into_iter().zip(ys.into_iter()))
}
