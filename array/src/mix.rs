use itertools::iproduct;

#[inline]
pub fn mix<X: Clone, Y: Clone, Z,
    const X_SIZE: usize,
    const Y_SIZE: usize,
    const Z_SIZE: usize>(
    xs: [X; X_SIZE],
    ys: [Y; Y_SIZE],
    f: impl Fn(X, Y) -> Z
) -> [Z; Z_SIZE] {
    array_operation!(Z_SIZE, f, iproduct!(xs, ys))
}

