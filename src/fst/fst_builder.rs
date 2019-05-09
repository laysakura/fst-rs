use super::{Fst, FstBuilder};

impl<Label: Ord + Clone> FstBuilder<Label> {
    pub fn new() -> Self {
        Self { a: vec![] }
    }

    pub fn push<Arr: AsRef<[Label]>>(&mut self, word: Arr) {}

    pub fn build(&self) -> Fst<Label> {
        Fst { a: vec![] }
    }
}
