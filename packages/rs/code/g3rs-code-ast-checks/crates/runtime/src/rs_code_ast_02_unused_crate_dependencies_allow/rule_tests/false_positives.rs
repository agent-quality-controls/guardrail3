use super::helpers::check_source;

#[test]
fn ignores_other_crate_level_allows() {
    let results = check_source("src/lib.rs", "#![allow(dead_code)]\nfn probe() {}\n", false);

    assert!(results.is_empty(), "{results:#?}");
}
