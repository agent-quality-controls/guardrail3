use guardrail3_domain_modules::clippy::build_clippy_toml;

use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn skips_query_as_inventory_when_garde_missing_for_root() {
    let root = temp_root("rs-garde-09-gating");
    let clippy_toml = build_clippy_toml("service", false, true, "", "");

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

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-09")
        .collect();
    assert!(results.is_empty());

    std::fs::remove_dir_all(root).expect("cleanup");
}
