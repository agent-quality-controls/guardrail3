use guardrail3_app_rs_family_garde_assertions::rs_garde_09_query_as_inventory as assertions;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn inventories_query_as_usage() {
    let root = temp_root("rs-garde-09-golden");
    let clippy_toml = build_clippy_toml("service", false, true, "", "");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["main.rs", "db.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            ("src/main.rs", "fn main() {}"),
            (
                "src/db.rs",
                r#"
use sqlx::query_as as qa;

fn load() {
    let _row = sqlx::query_as!(User, "select 1");
    let _row2 = qa!(User, "select 2");
}
"#,
            ),
        ],
        root.clone(),
    );

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 2, "unexpected RS-GARDE-09 findings: {findings:#?}");
    assertions::assert_inventory_hit(
        &results,
        "src/db.rs",
        5,
        "`sqlx::query_as` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit.",
    );
    assertions::assert_inventory_hit(
        &results,
        "src/db.rs",
        6,
        "`qa` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit.",
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn inventories_query_as_unchecked_usage() {
    let root = temp_root("rs-garde-09-unchecked");
    let clippy_toml = build_clippy_toml("service", false, true, "", "");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["main.rs", "db.rs"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"[workspace]
members = []
[workspace.dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("clippy.toml", clippy_toml.as_str()),
            ("guardrail3.toml", "[profile]\nname = \"service\"\n"),
            ("src/main.rs", "fn main() {}"),
            (
                "src/db.rs",
                r#"
use sqlx::query_as_unchecked as qau;

fn load() {
    let _row = sqlx::query_as_unchecked!(User, "select 1");
    let _row2 = qau!(User, "select 2");
}
"#,
            ),
        ],
        root.clone(),
    );

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 2, "unexpected RS-GARDE-09 findings: {findings:#?}");
    assertions::assert_inventory_hit(
        &results,
        "src/db.rs",
        5,
        "`sqlx::query_as_unchecked` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit.",
    );
    assertions::assert_inventory_hit(
        &results,
        "src/db.rs",
        6,
        "`qau` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit.",
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
