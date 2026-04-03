use guardrail3_app_rs_family_garde_assertions::rs_garde_13_context_validation_surface as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn ignores_fixed_rules_and_unused_context_annotations() {
    let root = temp_root("rs-garde-13-false-positives");
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

struct ValidationConfig {
    title_min: usize,
}

#[derive(Deserialize, Validate)]
#[garde(context(ValidationConfig as ctx))]
struct Input {
    #[garde(length(min = 1))]
    title: String,
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
fn ignores_qualified_function_paths_named_ctx() {
    let root = temp_root("rs-garde-13-qualified-ctx-path");
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
struct Input {
    #[garde(custom(function = crate::ctx))]
    title: String,
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
