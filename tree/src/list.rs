use crate::{binary_operation::*, fold::*, kids::*};

pub enum List<T> {
    Empty,
    Push(T, Box<List<T>>),
}

impl<T> Kids for List<T> {
    fn kids(&self) -> Vec<&Self> {
        match self {
            List::Empty => vec![],
            List::Push(_head, rest) => vec![&*rest],
        }
    }

    fn kids_mut(&mut self) -> Vec<&mut Self> {
        match self {
            List::Empty => vec![],
            List::Push(_head, rest) => vec![rest],
        }
    }
}

impl<T, R> FoldTop<R> for List<T>
where
    T: Copy + BinaryOperation<R>,
{
    fn fold_top(&self, value: &mut R) {
        match self {
            List::Push(head, _rest) => head.operate(value),
            _ => {}
        }
    }
}
