use guardrail3_app_rs_family_garde_assertions::rs_garde_09_query_as_inventory as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn inventories_query_as_only_for_the_owned_root() {
    let root = temp_root("rs-garde-09-multi-root");
    let clippy_toml = super::helpers::canonical_clippy_toml();

    for (rel, source) in [
        (
            "apps/lib/src/db.rs",
            r#"
fn load() {
    let _row = sqlx::query_as!(User, "select 1");
}
"#,
        ),
        ("apps/tool/src/db.rs", "fn load() {}\n"),
    ] {
        let abs = root.join(rel);
        std::fs::create_dir_all(
            abs.parent()
                .expect("fixture source path must have a parent directory"),
        )
        .expect("failed to create fixture source directory");
        std::fs::write(abs, source).expect("failed to write fixture source");
    }

    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &[])),
            ("apps", dir_entry(&["lib", "tool"], &[])),
            (
                "apps/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("apps/lib/src", dir_entry(&[], &["db.rs"])),
            (
                "apps/tool",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("apps/tool/src", dir_entry(&[], &["db.rs"])),
        ],
        vec![
            (
                "apps/lib/Cargo.toml",
                r#"[workspace]
members = []
[package]
name = "lib"
[dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("apps/lib/clippy.toml", clippy_toml.as_str()),
            (
                "apps/lib/guardrail3.toml",
                r#"[profile]
name = "service"

[[escape_hatches]]
family = "garde"
file = "apps/lib/src/db.rs"
kind = "sqlx_query_as"
selector = "sqlx::query_as@L3"
reason = "Temporary SQLx row mapping until validated DTO extraction lands."
"#,
            ),
            (
                "apps/tool/Cargo.toml",
                r#"[workspace]
members = []
[package]
name = "tool"
[dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("apps/tool/clippy.toml", clippy_toml.as_str()),
            (
                "apps/tool/guardrail3.toml",
                "[profile]\nname = \"service\"\n",
            ),
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);

    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 2);
    assertions::assert_rule_results(
        &results,
        &[
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                file: Some("apps/lib/src/db.rs"),
                inventory: Some(false),
                line: Some(3),
                title: Some("sqlx query_as requires validation review"),
                ..Default::default()
            },
            assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Warn),
                file: None,
                inventory: Some(false),
                title: Some("sqlx query_as count"),
                message: Some("`apps/lib/src/db.rs` has 1 sqlx query_as escape hatches."),
                ..Default::default()
            },
        ],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
