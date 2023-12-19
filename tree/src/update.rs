use crate::kids::*;

pub trait UpdateTop<R> {
    fn update_top(&mut self, value: &mut R);
}

pub trait UpdateRecursive<R> {
    fn update_recursive_pre(&mut self, value: &mut R);
    fn update_recursive_post(&mut self, value: &mut R);
}

impl<R, C> UpdateRecursive<R> for C
where
    C: UpdateTop<R> + Kids,
{
    fn update_recursive_pre(&mut self, value: &mut R) {
        self.update_top(value);
        for kid in self.kids_mut() {
            kid.update_recursive_pre(value);
        }
    }

    fn update_recursive_post(&mut self, value: &mut R) {
        for kid in self.kids_mut() {
            kid.update_recursive_post(value);
        }
        self.update_top(value)
    }
}
