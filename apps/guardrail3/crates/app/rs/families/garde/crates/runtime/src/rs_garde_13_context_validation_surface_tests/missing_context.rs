use guardrail3_app_rs_family_garde_assertions::rs_garde_13_context_validation_surface as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_when_ctx_usage_has_no_type_level_context() {
    let root = temp_root("rs-garde-13-missing");
    let source_rel = "src/input.rs";
    let source_abs = root.join(source_rel);
    let clippy_toml = super::super::canonical_clippy_toml();
    std::fs::create_dir_all(source_abs.parent().expect("fixture source path must have a parent directory")).expect("failed to create fixture source directory");
    std::fs::write(
        &source_abs,
        r#"
use serde::Deserialize;
use garde::Validate;

struct ValidationConfig {
    title_min: usize,
    title_max: usize,
}

#[derive(Deserialize, Validate)]
struct Input {
    #[garde(length(chars, min = ctx.title_min, max = ctx.title_max))]
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

    let results = super::super::run_family(&tree);
    let findings = assertions::findings(&results);
    assert_eq!(findings.len(), 1);
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some(source_rel),
            title: Some("boundary `Input` uses ctx without garde(context)"),
            message: Some(
                "Field `title` in validated boundary `Input` references `ctx` in a garde validator, but the boundary type is missing `#[garde(context(...))]`.",
            ),
            ..Default::default()
        }],
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
