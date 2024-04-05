pub mod binary_operation;
pub mod binary_tree;
pub mod fold;
pub mod kids;
pub mod list;
pub mod result;
pub mod trie;
pub mod update;

#[cfg(test)]
mod tests {
    use super::*;
    use binary_tree::*;
    use fold::*;
    use list::*;
    use result::*;
    use std::ops::AddAssign;
    use update::*;

    pub struct UpdateAdd<T>(T);

    impl<T> UpdateTop<UpdateAdd<T>> for BinaryTree<T>
    where
        T: Copy + AddAssign<T>,
    {
        fn update_top(&mut self, value: &mut UpdateAdd<T>) {
            match self {
                BinaryTree::Node(head, _) => *head += value.0,
                _ => {}
            }
        }
    }

    #[test]
    fn list() {
        let mut xs = List::Empty;
        xs = List::Push(1, Box::new(xs));
        xs = List::Push(2, Box::new(xs));
        xs = List::Push(3, Box::new(xs));
        assert_eq!(
            <List<i32> as Fold<AddResult<i32>>>::fold_pre(&xs),
            AddResult(6)
        );
        //assert_eq!(xs.fold_post(), SumResult(6));
    }

    #[test]
    fn tree() {
        use BinaryTree::*;
        /*
        let no_kids = Box::new([Empty, Empty]);
        let t1 = Node(1, no_kids.clone());
        let t2 = Node(2, no_kids.clone());
        let t = Node(3, Box::new([t1, t2]));
        */
        let e = Box::new(Empty);
        let t1 = Node(1, [e.clone(), e.clone()]);
        let t2 = Node(2, [e.clone(), e.clone()]);
        let mut t = Node(3, [Box::new(t1), Box::new(t2)]);
        let mut v = UpdateAdd(2);
        t.update_recursive_pre(&mut v);
        assert_eq!(
            <BinaryTree<i32> as Fold<AddResult<i32>>>::fold_pre(&t),
            AddResult(6 + 3 * v.0)
        );

        //assert_eq!(t.fold_pre(), R(4));
        //assert_eq!(t.fold_post(), R(2));
    }
}
