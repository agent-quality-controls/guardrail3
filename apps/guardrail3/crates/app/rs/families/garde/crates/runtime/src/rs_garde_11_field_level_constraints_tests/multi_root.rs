use guardrail3_app_rs_family_garde_assertions::rs_garde_11_field_level_constraints as assertions;
use guardrail3_domain_report::Severity;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn local_boundary_gap_only_errors_for_owned_root() {
    let root = temp_root("rs-garde-11-multi-root");
    let local_rel = "vendor/lib/src/input.rs";
    let shared_rel = "src/input.rs";
    let clippy_toml = super::super::canonical_clippy_toml();
    let local_abs = root.join(local_rel);
    let shared_abs = root.join(shared_rel);
    std::fs::create_dir_all(local_abs.parent().expect("parent")).expect("mkdir local");
    std::fs::create_dir_all(shared_abs.parent().expect("parent")).expect("mkdir shared");
    std::fs::write(
        &shared_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct SharedInput {
    #[garde(length(min = 1))]
    name: String,
}
"#,
    )
    .expect("write shared");
    std::fs::write(
        &local_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

#[derive(Deserialize, Validate)]
struct LocalInput {
    name: String,
}
"#,
    )
    .expect("write local");

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

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(local_rel),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("cleanup");
}
