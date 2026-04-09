use super::helpers::check_source;

#[test]
fn skips_missing_reason() {
    let results = check_source("src/lib.rs", "#[allow(dead_code)]\nfn probe() {}\n", false);

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_weak_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[allow(dead_code)] // reason: temp\nfn probe() {}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
