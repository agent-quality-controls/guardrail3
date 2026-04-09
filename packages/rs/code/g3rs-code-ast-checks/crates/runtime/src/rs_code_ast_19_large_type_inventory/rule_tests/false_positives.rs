use super::helpers::check_source;

#[test]
fn skips_small_types() {
    let results = check_source(
        "src/lib.rs",
        "struct Small { a: u8 }\nenum E { A, B }\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
