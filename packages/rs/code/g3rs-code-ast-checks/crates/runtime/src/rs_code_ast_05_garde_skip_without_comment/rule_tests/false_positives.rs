use super::helpers::check_source;

#[test]
fn skips_exempt_garde_skip_types() {
    let results = check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)]\n    enabled: bool,\n}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn skips_non_exempt_garde_skip_with_comment() {
    let results = check_source(
        "src/lib.rs",
        "struct Form {\n    #[garde(skip)] // reason: validated upstream boundary\n    token: String,\n}\n",
        false,
    );

    assert!(results.is_empty(), "{results:#?}");
}
