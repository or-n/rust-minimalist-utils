use crate::kids::*;

pub trait FoldTop<R> {
    fn fold_top(&self, value: &mut R);
}

pub trait FoldRecursive<R> {
    fn fold_recursive_pre(&self, value: &mut R);
    fn fold_recursive_post(&self, value: &mut R);
}

impl<R, C> FoldRecursive<R> for C
where
    C: FoldTop<R> + Kids,
{
    fn fold_recursive_pre(&self, value: &mut R) {
        self.fold_top(value);
        for kid in self.kids() {
            kid.fold_recursive_pre(value);
        }
    }

    fn fold_recursive_post(&self, value: &mut R) {
        for kid in self.kids() {
            kid.fold_recursive_post(value);
        }
        self.fold_top(value)
    }
}

pub trait Fold<R> {
    fn fold_pre(&self) -> R;
    fn fold_post(&self) -> R;
}

impl<R, C> Fold<R> for C
where
    C: FoldRecursive<R>,
    R: Default,
{
    fn fold_pre(&self) -> R {
        let mut value = R::default();
        self.fold_recursive_pre(&mut value);
        value
    }

    fn fold_post(&self) -> R {
        let mut value = R::default();
        self.fold_recursive_post(&mut value);
        value
    }
}
