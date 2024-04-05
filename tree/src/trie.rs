pub struct Trie<Edge> {
    valid: bool,
    deeper: Vec<(Edge, Trie<Edge>)>,
}

impl<Edge: PartialEq> Trie<Edge> {
    pub fn new() -> Self {
        Trie {
            valid: false,
            deeper: vec![],
        }
    }

    pub fn insert(&mut self, iter: &mut dyn Iterator<Item = Edge>) {
        match iter.next() {
            Some(iter_edge) => match self.step_mut(&iter_edge) {
                Some(subtrie) => subtrie.insert(iter),
                _ => {
                    let mut subtrie = Trie::new();
                    subtrie.insert(iter);
                    self.deeper.push((iter_edge, subtrie));
                }
            },
            _ => self.valid = true,
        }
    }

    pub fn step_mut(&mut self, step_edge: &Edge) -> Option<&mut Self> {
        self.deeper
            .iter_mut()
            .find(|(edge, _)| edge == step_edge)
            .map(|(_, subtrie)| subtrie)
    }

    pub fn step(&self, step_edge: &Edge) -> Option<&Self> {
        self.deeper
            .iter()
            .find(|(edge, _)| edge == step_edge)
            .map(|(_, subtrie)| subtrie)
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }
}
