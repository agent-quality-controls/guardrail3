use super::helpers::check_source;

#[test]
fn skips_canonical_test_sidecar() {
    let results = check_source(
        "src/lib.rs",
        "#[cfg(test)]\n#[path = \"rule_tests/mod.rs\"]\nmod rule_tests;\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn errors_on_weak_reason() {
    let results = check_source(
        "src/lib.rs",
        "#[path = \"generated.rs\"] // reason: temp\nmod generated;\n",
        false,
    );

    assert_eq!(results.len(), 1, "{results:#?}");
    assert_eq!(results[0].id(), "RS-CODE-24");
    assert_eq!(results[0].title(), "#[path] reason too weak");
}
