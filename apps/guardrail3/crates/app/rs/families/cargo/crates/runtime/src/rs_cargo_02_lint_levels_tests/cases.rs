use super::{
    FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS, check_results, entry, has_result,
    rule_results, tree,
};

#[test]
fn stricter_than_baseline_is_accepted_silently() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    )
    .replace(
        r#"missing_debug_implementations = "warn""#,
        r#"missing_debug_implementations = "deny""#,
    );

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-02");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}

#[test]
fn weaker_levels_are_errors() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    )
    .replace(r#"warnings = "deny""#, r#"warnings = "warn""#);

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    assert!(has_result(&results, "RS-CARGO-02", |result| {
        result.title == "lint `warnings` weakens policy"
    }));
}
