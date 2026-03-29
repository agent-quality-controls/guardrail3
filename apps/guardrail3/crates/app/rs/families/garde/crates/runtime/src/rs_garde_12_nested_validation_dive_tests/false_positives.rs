use guardrail3_app_rs_family_garde_assertions::rs_garde_12_nested_validation_dive as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn ignores_non_nested_fields_and_non_validated_custom_types() {
    let root = temp_root("rs-garde-12-false-positives");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

struct ExternalPayload {
    title: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(length(min = 1))]
    title: String,
    payload: ExternalPayload,
}
"#,
    )
    .expect("write");

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
    assert!(findings.is_empty());
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn ignores_foreign_qualified_type_even_when_other_root_defines_matching_name() {
    let root = temp_root("rs-garde-12-foreign-qualified");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct Input {
    profile: external::Profile,
}
"#,
    )
    .expect("write");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["src", "vendor/standalone", "vendor/standalone/src"],
                    &["Cargo.toml", "clippy.toml", "guardrail3.toml"],
                ),
            ),
            ("src", dir_entry(&[], &["input.rs"])),
            (
                "vendor/standalone",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("vendor/standalone/src", dir_entry(&[], &["lib.rs"])),
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
            (
                "vendor/standalone/Cargo.toml",
                r#"[package]
name = "standalone"
version = "0.1.0"

[dependencies]
garde = "0.22"
"#,
            ),
            ("vendor/standalone/clippy.toml", clippy_toml.as_str()),
            (
                "vendor/standalone/src/lib.rs",
                r#"
use garde::Validate;

#[derive(Validate)]
pub struct Profile {
    #[garde(length(min = 1))]
    name: String,
}
"#,
            ),
        ],
        root.clone(),
    );

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert!(findings.is_empty(), "expected no RS-GARDE-12 findings: {findings:#?}");
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn ignores_skipped_nested_validated_fields() {
    let root = temp_root("rs-garde-12-skip-nested");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
    std::fs::write(
        &source_abs,
        r#"
use garde::Validate;
use serde::Deserialize;

#[derive(Deserialize, Validate)]
struct Metadata {
    #[garde(length(min = 1))]
    name: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(skip)]
    metadata: Metadata,
}
"#,
    )
    .expect("write");

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
    assert!(findings.is_empty());
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}
