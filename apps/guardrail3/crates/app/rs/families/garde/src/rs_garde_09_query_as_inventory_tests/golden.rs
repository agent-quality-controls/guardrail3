use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;

use super::super::super::test_support::{dir_entry, project_tree, temp_root};

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

    let results = crate::test_support::run_family(&tree);

    let mut rs_garde_09_results: Vec<_> = results
        .into_iter()
        .filter(|r| r.id == "RS-GARDE-09")
        .collect();

    rs_garde_09_results.sort_by_key(|r| r.line);

    assert_eq!(rs_garde_09_results.len(), 2);

    assert_eq!(rs_garde_09_results[0].severity, Severity::Info);
    assert_eq!(rs_garde_09_results[0].file.as_deref(), Some("src/db.rs"));
    assert_eq!(rs_garde_09_results[0].line, Some(5));
    assert!(rs_garde_09_results[0].inventory);
    assert_eq!(
        rs_garde_09_results[0].message,
        "`sqlx::query_as` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit."
    );

    assert_eq!(rs_garde_09_results[1].severity, Severity::Info);
    assert_eq!(rs_garde_09_results[1].file.as_deref(), Some("src/db.rs"));
    assert_eq!(rs_garde_09_results[1].line, Some(6));
    assert!(rs_garde_09_results[1].inventory);
    assert_eq!(
        rs_garde_09_results[1].message,
        "`qa` bypasses derive-based garde boundary checks. Review the target type and ensure validated input handling is explicit."
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
