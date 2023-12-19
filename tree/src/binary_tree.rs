use crate::{binary_operation::*, fold::*, kids::*};

#[derive(Clone)]
pub enum BinaryTree<T> {
    Empty,
    Node(T, [Box<BinaryTree<T>>; 2]),
}

impl<T> Kids for BinaryTree<T> {
    fn kids(&self) -> Vec<&Self> {
        match self {
            BinaryTree::Empty => vec![],
            BinaryTree::Node(_value, [left, right]) => vec![&*left, &*right],
        }
    }

    fn kids_mut(&mut self) -> Vec<&mut Self> {
        match self {
            BinaryTree::Empty => vec![],
            BinaryTree::Node(_value, [left, right]) => vec![left, right],
        }
    }
}

impl<T, R> FoldTop<R> for BinaryTree<T>
where
    T: Copy + BinaryOperation<R>,
{
    fn fold_top(&self, value: &mut R) {
        match self {
            BinaryTree::Node(head, _rest) => head.operate(value),
            _ => {}
        }
    }
}
