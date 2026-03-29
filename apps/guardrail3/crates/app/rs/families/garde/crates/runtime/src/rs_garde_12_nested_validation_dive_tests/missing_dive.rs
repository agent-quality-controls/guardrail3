use guardrail3_app_rs_family_garde_assertions::rs_garde_12_nested_validation_dive as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_nested_validated_field_lacks_dive() {
    let root = temp_root("rs-garde-12-missing");
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
struct Payload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    payload: Payload,
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
    assert_eq!(findings.len(), 1, "unexpected RS-GARDE-12 findings: {findings:#?}");
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(13),
        Some("nested validated field `payload` missing garde(dive)"),
        Some(
            "Field `payload` in validated boundary `Input` points at validated nested type `Payload` but is missing `#[garde(dive)]`. Nested validated fields must opt into recursive garde validation.",
        ),
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}

#[test]
fn errors_when_array_of_nested_validated_field_lacks_dive() {
    let root = temp_root("rs-garde-12-array-missing");
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
struct Payload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct Input {
    payloads: [Payload; 2],
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
    assert_eq!(findings.len(), 1, "unexpected RS-GARDE-12 findings: {findings:#?}");
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(13),
        Some("nested validated field `payloads` missing garde(dive)"),
        None,
    );
    assertions::assert_field_level_constraints_quiet(&results);

    std::fs::remove_dir_all(root).expect("cleanup");
}
