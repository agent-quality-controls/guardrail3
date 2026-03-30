use guardrail3_app_rs_family_garde_assertions::rs_garde_08_enum_derive_validate as assertions;
use test_support::{dir_entry, project_tree, temp_root};

fn run_enum_boundary(source: &str) -> Vec<assertions::CheckResult> {
    let root = temp_root("rs-garde-08-derive-variants");
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
fn errors_when_parser_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use clap::Parser;

#[derive(Parser)]
enum Input {
    Variant(String),
}
"#,
    );

    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Enum `Input` derives Parser and has non-primitive payload fields, but does not derive `Validate`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_args_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use clap::Args;

#[derive(Args)]
enum Input {
    Variant(String),
}
"#,
    );

    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Enum `Input` derives Args and has non-primitive payload fields, but does not derive `Validate`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_from_row_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use sqlx::FromRow;

#[derive(FromRow)]
enum Input {
    Variant(String),
}
"#,
    );

    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Enum `Input` derives FromRow and has non-primitive payload fields, but does not derive `Validate`.",
            ),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_aliased_deserialize_enum_lacks_validate() {
    let results = run_enum_boundary(
        r#"
use serde::Deserialize as De;

#[derive(De)]
enum Input {
    Variant(String),
}
"#,
    );

    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            message: Some(
                "Enum `Input` derives De and has non-primitive payload fields, but does not derive `Validate`.",
            ),
            ..Default::default()
        }],
    );
}
