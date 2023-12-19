pub trait FoldNonempty<T> {
    fn fold_nonempty(self, f: impl Fn(T, T) -> T) -> T;
}

impl<T, const SIZE: usize> FoldNonempty<T> for [T; SIZE] {
    #[inline]
    fn fold_nonempty(self, f: impl Fn(T, T) -> T) -> T {
        let mut iter = self.into_iter();
        let initial = match iter.next() {
            Some(first) => first,
            None => panic!("Attempted to fold_nonempty on an empty array."),
        };
        iter.fold(initial, f)
    }
}
