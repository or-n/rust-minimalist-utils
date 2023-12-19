pub trait Eval<T> {
    fn score(&self) -> T;
}
