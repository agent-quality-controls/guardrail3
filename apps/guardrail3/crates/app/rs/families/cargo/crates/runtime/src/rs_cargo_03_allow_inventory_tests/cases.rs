use super::{
    FULL_STANDALONE_CLIPPY_LINTS, FULL_STANDALONE_RUST_LINTS, assert_expected_inventory,
    assert_result_count, check_results, entry, rule_results, tree,
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
    assert_result_count(&results, 9);
    assert_expected_inventory(&results);
    assert!(rule.iter().all(|result| result.inventory));
    assert!(
        rule.iter()
            .any(|result| result.title == "allow inventory: `multiple_crate_versions`")
    );
}
