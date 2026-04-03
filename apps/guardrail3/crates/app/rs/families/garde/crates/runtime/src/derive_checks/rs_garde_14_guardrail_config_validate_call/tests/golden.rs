use guardrail3_app_rs_family_garde_assertions::rs_garde_14_guardrail_config_validate_call as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn stays_quiet_when_guardrail_config_parse_is_validated() {
    let root = temp_root("rs-garde-14-golden");
    let source_rel = "src/load_config.rs";
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
use garde::Validate;
use guardrail3_domain_config::types::GuardrailConfig;

fn load_config(content: &str) -> Option<GuardrailConfig> {
    let cfg = toml::from_str(content).ok()?;
    cfg.validate().ok()?;
    Some(cfg)
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
            ("src", dir_entry(&[], &["load_config.rs"])),
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
    assertions::assert_rule_quiet(&results);

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
