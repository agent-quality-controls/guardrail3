use guardrail3_app_rs_family_garde_assertions::rs_garde_09_query_as_inventory as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn skips_query_as_inventory_when_garde_missing_for_root() {
    let root = temp_root("rs-garde-09-gating");
    let clippy_toml = super::super::canonical_clippy_toml();

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []\n"),
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            (
                "src/db.rs",
                r#"
fn load() {
    let _row = sqlx::query_as!(User, "select 1");
}
"#,
            ),
        ],
        root.clone(),
    );

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert!(findings.is_empty());
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}
