#[test]
fn skips_missing_reason() {
    let results =
        super::super::check_source("src/lib.rs", "#[allow(dead_code)]\nfn probe() {}\n", false);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_weak_reason() {
    let results = super::super::check_source(
        "src/lib.rs",
        "#[allow(dead_code)] // reason: temp\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
