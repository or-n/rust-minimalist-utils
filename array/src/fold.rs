pub trait FoldNonempty<T> {    
    fn fold_nonempty(self, f: impl Fn(T, T) -> T) -> T;
}

impl<T, const SIZE: usize> FoldNonempty<T> for [T; SIZE] {
    #[inline]
    fn fold_nonempty(self, f: impl Fn(T, T) -> T) -> T {
        let mut iter = self.into_iter();
        let mut acc = match iter.next() {
            Some(first) => first,
            None => panic!("Array is empty."),
        };
        for item in iter {
            acc = f(acc, item);
        }
        acc
    }
}
