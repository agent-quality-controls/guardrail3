use guardrail3_domain_report::{CheckResult, Severity};
use test_support::{dir_entry, project_tree, temp_root};

fn findings<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results.iter().filter(|result| result.id() == id).collect()
}

#[test]
fn routes_struct_boundary_rule_through_ast_package() {
    let root = temp_root("rs-garde-ast-bridge-struct");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::helpers::canonical_clippy_toml();
    std::fs::create_dir_all(
        source_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        "use serde::Deserialize;\n\n#[derive(Deserialize)]\nstruct Input {\n    name: String,\n}\n",
    )
    .expect("failed to write fixture source");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["input.rs"])),
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
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);
    let garde_05 = findings(&results, "RS-GARDE-05");
    assert_eq!(garde_05.len(), 1, "unexpected RS-GARDE-05 results: {garde_05:#?}");
    assert_eq!(garde_05[0].severity(), Severity::Error);
    assert_eq!(garde_05[0].file(), Some(source_rel));
    assert_eq!(garde_05[0].title(), "struct `Input` missing Validate derive");
    assert!(
        garde_05[0]
            .message()
            .contains("does not derive garde's `Validate`")
    );
    assert!(findings(&results, "RS-GARDE-10").is_empty());

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn routes_query_as_rule_through_ast_package_with_guardrail_reason_lookup() {
    let root = temp_root("rs-garde-ast-bridge-query-as");
    let source_rel = "src/db.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::helpers::canonical_clippy_toml();
    std::fs::create_dir_all(
        source_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        "use sqlx::query_as as qa;\n\nfn load() {\n    let _row = sqlx::query_as!(User, \"select 1\");\n    let _row2 = qa!(User, \"select 2\");\n}\n",
    )
    .expect("failed to write fixture source");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
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
            (
                "guardrail3.toml",
                r#"[profile]
name = "service"

[[escape_hatches]]
family = "garde"
file = "src/db.rs"
kind = "sqlx_query_as"
selector = "sqlx::query_as@L4"
reason = "Temporary SQLx row mapping until validated DTO extraction lands."

[[escape_hatches]]
family = "garde"
file = "src/db.rs"
kind = "sqlx_query_as"
selector = "qa@L5"
reason = "Temporary SQLx row mapping until validated DTO extraction lands."
"#,
            ),
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);
    let garde_09 = findings(&results, "RS-GARDE-09");
    assert_eq!(garde_09.len(), 3, "unexpected RS-GARDE-09 results: {garde_09:#?}");
    assert!(garde_09.iter().any(|result| {
        result.severity() == Severity::Warn
            && result.file() == Some(source_rel)
            && result
                .message()
                .contains("`sqlx::query_as` bypasses derive-based garde boundary checks")
    }));
    assert!(garde_09.iter().any(|result| {
        result.severity() == Severity::Warn
            && result.file() == Some(source_rel)
            && result
                .message()
                .contains("`qa` bypasses derive-based garde boundary checks")
    }));
    assert!(garde_09.iter().any(|result| {
        result.severity() == Severity::Warn
            && result.title() == "sqlx query_as count"
            && result.message() == "`src/db.rs` has 2 sqlx query_as escape hatches."
    }));
    assert!(findings(&results, "RS-GARDE-10").is_empty());

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn malformed_guardrail_policy_blocks_ast_query_as_checks() {
    let root = temp_root("rs-garde-ast-bridge-guardrail-failure");
    let source_rel = "src/db.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::helpers::canonical_clippy_toml();
    std::fs::create_dir_all(
        source_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        "fn load() {\n    let _row = sqlx::query_as!(User, \"select 1\");\n}\n",
    )
    .expect("failed to write fixture source");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("src", dir_entry(&[], &["db.rs"])),
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
            ("guardrail3.toml", "[[broken"),
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);
    let garde_10 = findings(&results, "RS-GARDE-10");
    assert_eq!(garde_10.len(), 1, "unexpected RS-GARDE-10 results: {garde_10:#?}");
    assert!(garde_10[0]
        .message()
        .contains("Failed to parse guardrail3.toml for garde policy resolution"));
    assert!(findings(&results, "RS-GARDE-09").is_empty());

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
