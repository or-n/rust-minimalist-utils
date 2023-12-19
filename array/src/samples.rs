#[derive(Clone, Copy)]
pub struct RatioIndex(pub usize, pub usize);

pub fn in_(RatioIndex(i, _): RatioIndex) -> usize {
    i
}

pub fn out(RatioIndex(i, n): RatioIndex) -> usize {
    n - 1 - i
}

pub fn in_out(x: RatioIndex) -> usize {
    in_(x).min(out(x))
}

pub fn subsume<A: Copy, B: Copy, NewB>(
    a: &mut [A],
    b: &[B],
    f: impl Fn(A, NewB) -> A,
    modifier: impl Fn(B, RatioIndex) -> NewB
) {
    for i in 0..b.len() {
        let new_b = modifier(b[i], RatioIndex(i, b.len()));
        a[i] = f(a[i], new_b);
    }
}

pub fn modify<T: Copy>(
    data: &mut [T],
    modifier: impl Fn(T, RatioIndex) -> T
) {
    for i in 0..data.len() {
        data[i] = modifier(data[i], RatioIndex(i, data.len()));
    }
}

pub fn samples<T, const N: usize>(f: impl Fn(usize) -> T) -> [T; N] {
    uninitialized!(xs, N);
    for i in 0..N {
        xs[i] = f(i);
    }
    xs
}

#[macro_export]
macro_rules! samples {
    ($n:expr, $f: expr) => {
        samples::<_, {$n}>($f)
    };
}
