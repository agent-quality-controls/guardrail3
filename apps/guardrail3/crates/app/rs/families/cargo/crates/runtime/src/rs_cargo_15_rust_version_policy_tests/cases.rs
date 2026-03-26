use super::{
    FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS, check_results, entry, rule_results,
    tree,
};

#[test]
fn library_profile_missing_rust_version_is_error() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &manifest),
            ("pkg/guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-15");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].title, "library rust-version missing");
}

#[test]
fn non_library_missing_rust_version_is_inventory_only() {
    let manifest = format!(
        r#"
            [package]
            name = "helper"
            edition = "2024"

            {FULL_STANDALONE_RUST_LINTS}
            {FULL_STANDALONE_CLIPPY_LINTS}
        "#
    );
    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-15");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}

#[test]
fn library_profile_with_rust_version_is_inventory() {
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
        &[("pkg", entry(&[], &["Cargo.toml", "guardrail3.toml"]))],
        &[
            ("pkg/Cargo.toml", &manifest),
            ("pkg/guardrail3.toml", "[profile]\nname = \"library\"\n"),
        ],
    ));

    let rule = rule_results(&results, "RS-CARGO-15");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}
