#[test]
fn ignores_other_crate_level_allows() {
    let results =
        super::super::check_source("src/lib.rs", "#![allow(dead_code)]\nfn probe() {}\n", false);

    assert!(results.is_empty(), "{results:#?}");
}
