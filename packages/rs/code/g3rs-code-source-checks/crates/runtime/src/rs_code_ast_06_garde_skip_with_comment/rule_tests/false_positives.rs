
#[test]
fn skips_exempt_garde_skip_types() {
    let results = super::super::check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)] // reason: primitive passthrough\n    enabled: bool,\n}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
