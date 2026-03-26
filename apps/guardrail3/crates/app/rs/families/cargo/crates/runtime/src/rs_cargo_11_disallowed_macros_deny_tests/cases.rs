use super::{
    FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS, check_results, entry, rule_results,
    tree,
};

#[test]
fn disallowed_macros_deny_is_inventory() {
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

    let rule = rule_results(&results, "RS-CARGO-11");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert!(rule[0].inventory);
}

#[test]
fn missing_disallowed_macros_is_error() {
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
    .replace("    disallowed_macros = \"deny\"\n", "");
    let results = check_results(&tree(
        &[("pkg", entry(&[], &["Cargo.toml"]))],
        &[("pkg/Cargo.toml", &manifest)],
    ));

    let rule = rule_results(&results, "RS-CARGO-11");
    assert_eq!(rule.len(), 1, "unexpected results: {rule:#?}");
    assert_eq!(rule[0].title, "disallowed macros lint missing");
}
