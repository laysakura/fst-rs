pub mod fst;
pub mod fst_builder;

pub struct Fst<Label> {
    a: Vec<Label>,
}

pub struct FstBuilder<Label> {
    a: Vec<Label>,
}
