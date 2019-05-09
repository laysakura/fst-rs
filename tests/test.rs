use fst_rs::{Fst, FstBuilder};
use std::str;

fn build() -> Fst<u8> {
    let mut builder = FstBuilder::new();
    builder.push("すし");
    builder.push("すしや");
    builder.push("すしだね");
    builder.push("すしづめ");
    builder.push("すしめし");
    builder.push("すしをにぎる");
    builder.push("すし"); // Word `push`ed twice is just ignored.
    builder.push("🍣");

    builder.build() // Dense-Sparse Ratio
}

#[test]
fn lookup_key_test() {
    let fst = build();

    assert_eq!(fst.lookup_key("すし"), true);
    assert_eq!(fst.lookup_key("🍣"), true);
    assert_eq!(fst.lookup_key("🍜"), false);

    // TODO false-positive case
}

#[test]
fn lookup_range_test() {
    let fst = build();

    // [すし, すしや)
    let results_in_u8s: Vec<Vec<u8>> = fst.lookup_range("すし", true, "すしや", false);
    let results_in_str: Vec<&str> = results_in_u8s
        .iter()
        .map(|u8s| str::from_utf8(u8s).unwrap())
        .collect();

    assert_eq!(
        results_in_str,
        vec!["すし", "すしだね", "すしづめ", "すしめし",] // Sorted by `Vec<u8>`'s order
    );

    // TODO false-positive case
}
