use guardrail3_app_rs_family_garde_assertions::rs_garde_05_struct_derive_validate as assertions;
use test_support::{dir_entry, project_tree, temp_root};

fn run_struct_boundary(source: &str) -> Vec<assertions::CheckResult> {
    let root = temp_root("rs-garde-05-derive-variants");
    let source_abs = root.join("src/input.rs");
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(
        source_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create fixture source directory");
    std::fs::write(&source_abs, source).expect("failed to write fixture source");

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

    let results = super::super::run_family(&tree);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
    results
}

#[test]
fn errors_when_parser_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use clap::Parser;

#[derive(Parser)]
struct Input {
    name: String,
}
"#,
    );

    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Struct `Input` derives Parser but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_args_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use clap::Args;

#[derive(Args)]
struct Input {
    name: String,
}
"#,
    );

    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Struct `Input` derives Args but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_from_row_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use sqlx::FromRow;

#[derive(FromRow)]
struct Input {
    name: String,
}
"#,
    );

    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Struct `Input` derives FromRow but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_aliased_deserialize_struct_lacks_validate() {
    let results = run_struct_boundary(
        r#"
use serde::Deserialize as De;

#[derive(De)]
struct Input {
    name: String,
}
"#,
    );

    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Struct `Input` derives De but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
            ),
            ..Default::default()
        }],
    );
}
