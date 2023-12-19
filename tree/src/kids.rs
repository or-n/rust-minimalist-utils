pub trait Kids {
    fn kids(&self) -> Vec<&Self>;
    fn kids_mut(&mut self) -> Vec<&mut Self>;
}
