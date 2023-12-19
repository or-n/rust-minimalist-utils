use crate::iterate::many_iterator::*;

pub struct History1<T, F> where F: Fn(&mut T, &T) {
    pub step_limit: usize,
    pub a_is_new: bool,
    pub a: T,
    pub b: T,
    pub f: F
}

macro_rules! iterate {
    ($steps:expr, $f:expr, $a_is_new:expr, $a:expr, $b:expr) => {{
        for _ in 0..$steps / 2 {
            $f(&mut $b, &$a);
            $f(&mut $a, &$b);
        }
        if $steps % 2 == 0 {
            Some(&$a)
        } else {
            $f(&mut $b, &$a);
            $a_is_new = !$a_is_new;
            Some(&$b)
        }
    }}
}

impl<T, F> ManyIterator<T> for History1<T, F>
where
    F: Fn(&mut T, &T),
{
    fn many_next(&mut self, steps: usize) -> Option<&T> {
        if steps > self.step_limit {
            return None
        }
        self.step_limit -= steps;
        if self.a_is_new {
            iterate!(steps, self.f, self.a_is_new, self.a, self.b)
        } else {
            iterate!(steps, self.f, self.a_is_new, self.b, self.a)
        }
    }
}
