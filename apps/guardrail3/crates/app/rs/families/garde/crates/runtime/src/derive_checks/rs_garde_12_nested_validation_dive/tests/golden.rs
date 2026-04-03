use guardrail3_app_rs_family_garde_assertions::rs_garde_12_nested_validation_dive as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn stays_quiet_when_nested_validated_fields_use_dive() {
    let root = temp_root("rs-garde-12-golden");
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
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct Payload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(dive)]
    payload: Payload,
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

    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert!(findings.is_empty());
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn stays_quiet_when_array_of_nested_validated_fields_use_dive() {
    let root = temp_root("rs-garde-12-array-golden");
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
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct Payload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(dive)]
    payloads: [Payload; 2],
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

    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert!(
        findings.is_empty(),
        "expected no RS-GARDE-12 findings: {findings:#?}"
    );
    assertions::assert_rule_quiet(&results);
    assertions::assert_field_level_constraints_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
