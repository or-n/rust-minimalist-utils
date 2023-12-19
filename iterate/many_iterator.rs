
pub trait ManyIterator<T> {
    fn many_next(&mut self, steps: usize) -> Option<&T>;
}
