use crate::test_fixtures::canonical_clippy_toml;
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_validated_boundary_field_has_no_real_garde_rule() {
    let root = temp_root("rs-garde-11-missing");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
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

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-11")
        .collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(source_rel));
    assert_eq!(results[0].line, Some(7));
    assert_eq!(
        results[0].title,
        "boundary field `name` missing garde validator"
    );
    assert_eq!(
        results[0].message,
        "Field `name` in validated boundary `Input` has type `String` but no meaningful garde validator. Add a field-level garde rule such as `length`, `range`, `url`, or another explicit validator."
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn does_not_treat_foreign_qualified_type_as_local_nested_validated() {
    let root = temp_root("rs-garde-11-foreign-qualified");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
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

    let results = crate::test_fixtures::run_family(&tree);
    let rs_garde_11_results: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-GARDE-11")
        .collect();
    assert_eq!(rs_garde_11_results.len(), 1);
    assert_eq!(rs_garde_11_results[0].severity, Severity::Error);
    assert_eq!(rs_garde_11_results[0].file.as_deref(), Some(source_rel));
    assert_eq!(rs_garde_11_results[0].line, Some(7));
    assert_eq!(
        rs_garde_11_results[0].title,
        "boundary field `profile` missing garde validator"
    );

    assert!(
        results.iter().all(|result| result.id != "RS-GARDE-12"),
        "foreign qualified type should not invent nested validated ownership: {results:#?}"
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn custom_types_with_map_suffix_still_need_field_validators() {
    let root = temp_root("rs-garde-11-custom-map-suffix");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("parent")).expect("mkdir");
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

    let results: Vec<_> = crate::test_fixtures::run_family(&tree)
        .into_iter()
        .filter(|result| result.id == "RS-GARDE-11")
        .collect();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file.as_deref(), Some(source_rel));
    assert_eq!(results[0].line, Some(11));
    assert_eq!(
        results[0].title,
        "boundary field `assets` missing garde validator"
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
