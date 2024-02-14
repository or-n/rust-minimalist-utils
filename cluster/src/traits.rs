use array::uninitialized;

pub trait Distance<T> {
    fn distance(a: &T, b: &T) -> Self;
}

pub trait Mean<T> {
    fn mean(&self) -> T;
}

impl<T> Mean<T> for Vec<T>
where
    T: Copy + std::iter::Sum<T> + std::ops::Div<usize, Output = T>,
{
    fn mean(&self) -> T {
        self.iter().cloned().sum::<T>() / self.len()
    }
}

pub trait Sample<T, const K: usize> {
    fn sample(&self) -> Option<[T; K]>;
}

impl<T: Copy, const K: usize> Sample<T, K> for Vec<T> {
    fn sample(&self) -> Option<[T; K]> {
        if self.len() < K {
            return None;
        }
        uninitialized!(sample, K);
        for i in 0..K {
            sample[i] = self[i * (self.len() - 1) / (K - 1)]
        }
        Some(sample)
    }
}
