
pub struct _PrefixSublens<T, const N: usize> {
    pattern: [T; N],
    sublens: [usize; N]
}

impl<T: Eq + Clone, const N: usize> _PrefixSublens<T, N> {

    pub fn _new(pattern: [T; N]) -> Self {
        uninitialized!(sublens, N);
        sublens[0] = 0;
        let mut p = Self { pattern, sublens };
        for i in 1..p.pattern.len() {
            p.sublens[i] = p._query(p.sublens[i - 1], p.pattern[i].clone())
        }
        p
    }
    
    pub fn _query(&self, mut len: usize, item: T) -> usize {
        if len == self.pattern.len() {
            len = self.sublens[len - 1]
        }
        while len > 0 && self.pattern[len] != item {
            len = self.sublens[len - 1]
        }
        if self.pattern[len] == item {
            len += 1
        }
        len
    }
}
