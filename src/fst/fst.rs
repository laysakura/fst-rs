use super::Fst;

impl<Label> Fst<Label> {
    pub fn lookup_key<Arr: AsRef<[Label]>>(&self, query: Arr) -> bool {
        true
    }

    pub fn lookup_range<Arr: AsRef<[Label]>>(
        &self,
        query_left: Arr,
        inclusive_left: bool,
        query_right: Arr,
        inclusive_right: bool,
    ) -> Vec<Vec<Label>> {
        vec![]
    }
}
