use guardrail3_app_rs_family_garde_assertions::rs_garde_12_nested_validation_dive as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_missing_dive_only_errors_for_owned_root() {
    let root = temp_root("rs-garde-12-multi-root");
    let local_rel = "vendor/lib/src/input.rs";
    let shared_rel = "src/input.rs";
    let clippy_toml = super::helpers::canonical_clippy_toml();
    let local_abs = root.join(local_rel);
    let shared_abs = root.join(shared_rel);
    std::fs::create_dir_all(
        local_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create local fixture source directory");
    std::fs::create_dir_all(
        shared_abs
            .parent()
            .expect("fixture source path must have a parent directory"),
    )
    .expect("failed to create shared fixture source directory");
    std::fs::write(
        &shared_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct SharedPayload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct SharedInput {
    #[garde(dive)]
    payload: SharedPayload,
}
"#,
    )
    .expect("failed to write shared fixture source");
    std::fs::write(
        &local_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct LocalPayload {
    #[garde(length(min = 1))]
    title: String,
}

#[derive(Deserialize, Validate)]
struct LocalInput {
    payload: LocalPayload,
}
"#,
    )
    .expect("failed to write local fixture source");

    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["src", "vendor"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("src", dir_entry(&[], &["input.rs"])),
            ("vendor", dir_entry(&["lib"], &[])),
            (
                "vendor/lib",
                dir_entry(&["src"], &["Cargo.toml", "clippy.toml", "guardrail3.toml"]),
            ),
            ("vendor/lib/src", dir_entry(&[], &["input.rs"])),
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
                "vendor/lib/Cargo.toml",
                r#"[package]
name = "lib"
[dependencies]
garde = { version = "0.22", features = ["derive"] }
"#,
            ),
            ("vendor/lib/clippy.toml", clippy_toml.as_str()),
            (
                "vendor/lib/guardrail3.toml",
                "[profile]\nname = \"service\"\n",
            ),
        ],
        root.clone(),
    );

    let results = super::helpers::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some(local_rel),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
