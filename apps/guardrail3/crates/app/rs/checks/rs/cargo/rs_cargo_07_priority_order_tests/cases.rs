use super::super::super::test_support::{
    FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS, check_results, entry, rule_results,
    tree,
};

#[test]
fn clean_specific_priorities_are_inventory() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = "1.85"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    );

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-07");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}

#[test]
fn negative_specific_priority_warns() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"
            rust-version = "1.85"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    )
    .replace(
        r#"unwrap_used = "deny""#,
        r#"unwrap_used = { level = "deny", priority = -2 }"#,
    );

    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-07");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(
        rule[0].title,
        "specific lint `unwrap_used` has negative priority"
    );
}
