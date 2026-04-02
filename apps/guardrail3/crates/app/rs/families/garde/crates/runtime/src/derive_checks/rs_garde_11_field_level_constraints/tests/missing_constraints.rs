use guardrail3_app_rs_family_garde_assertions::rs_garde_11_field_level_constraints as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_validated_boundary_field_has_no_real_garde_rule() {
    let root = temp_root("rs-garde-11-missing");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
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
    let findings = assertions::findings(&results);
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-11 findings: {findings:#?}"
    );
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(7),
        Some("boundary field `name` missing garde validator"),
        Some(
            "Field `name` in validated boundary `Input` has type `String` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator.",
        ),
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn does_not_treat_foreign_qualified_type_as_local_nested_validated() {
    let root = temp_root("rs-garde-11-foreign-qualified");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
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
struct Input {
    profile: external::Profile,
}
"#,
    )
    .expect("failed to write fixture source");

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
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-11 findings: {findings:#?}"
    );
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(7),
        Some("boundary field `profile` missing garde validator"),
        None,
    );
    assertions::assert_nested_dive_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn custom_types_with_map_suffix_still_need_field_validators() {
    let root = temp_root("rs-garde-11-custom-map-suffix");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
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

struct AssetMap {
    name: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    assets: AssetMap,
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
    assert_eq!(
        findings.len(),
        1,
        "unexpected RS-GARDE-11 findings: {findings:#?}"
    );
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(11),
        Some("boundary field `assets` missing garde validator"),
        None,
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
