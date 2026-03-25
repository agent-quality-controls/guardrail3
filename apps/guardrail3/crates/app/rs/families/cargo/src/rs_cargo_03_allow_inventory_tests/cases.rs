use super::super::super::lint_support::EXPECTED_CLIPPY_ALLOW;
use super::super::super::test_support::{
    FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS, check_results, entry, rule_results,
    tree,
};

#[test]
fn inventories_every_approved_allow_entry() {
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

    let rule = rule_results(&results, "RS-CARGO-03");
    assert_eq!(
        rule.len(),
        EXPECTED_CLIPPY_ALLOW.len(),
        "unexpected inventory: {rule:#?}"
    );
    assert!(rule.iter().all(|result| result.inventory));
    assert!(
        rule.iter()
            .any(|result| result.title == "allow inventory: `multiple_crate_versions`")
    );
}
