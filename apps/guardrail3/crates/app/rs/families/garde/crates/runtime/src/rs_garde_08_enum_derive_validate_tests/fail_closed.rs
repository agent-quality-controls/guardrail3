use guardrail3_app_rs_family_garde_assertions::rs_garde_08_enum_derive_validate as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_enum_boundary_type_lacks_validate() {
    let root = temp_root("rs-garde-08-fail-closed");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
enum Input {
    Variant(String),
}
"#,
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

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some(source_rel),
            line: Some(4),
            title: Some("enum `Input` missing Validate derive"),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn non_garde_validate_derive_does_not_suppress_enum_error() {
    let root = temp_root("rs-garde-08-fake-validate");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize, fake::Validate)]
enum Input {
    Variant(String),
}
"#,
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

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some(source_rel),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
