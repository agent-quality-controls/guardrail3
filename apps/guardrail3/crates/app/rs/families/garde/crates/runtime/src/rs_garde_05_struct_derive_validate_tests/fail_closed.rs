use guardrail3_app_rs_family_garde_assertions::rs_garde_05_struct_derive_validate as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_struct_boundary_type_lacks_validate() {
    let root = temp_root("rs-garde-05-fail-closed");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    name: String,
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
    let _ = assertions::findings(&results);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            title: Some("struct `Input` missing Validate derive"),
            file: Some(source_rel),
            message: Some(
                "Struct `Input` derives Deserialize but does not derive `Validate`. Non-primitive input boundary structs must derive garde validation.",
            ),
            line: Some(4),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn non_garde_validate_derive_does_not_suppress_struct_error() {
    let root = temp_root("rs-garde-05-fake-validate");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;

#[derive(Deserialize, fake::Validate)]
struct Input {
    name: String,
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
    let _ = assertions::findings(&results);
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
