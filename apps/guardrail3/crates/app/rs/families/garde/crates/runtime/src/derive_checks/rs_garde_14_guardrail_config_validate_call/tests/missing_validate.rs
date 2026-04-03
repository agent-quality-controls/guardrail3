use guardrail3_app_rs_family_garde_assertions::rs_garde_14_guardrail_config_validate_call as assertions;
use test_support::{dir_entry, project_tree, temp_root};

#[test]
fn errors_on_inferred_toml_from_str_without_validate() {
    let root = temp_root("rs-garde-14-inferred");
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
use guardrail3_domain_config::types::GuardrailConfig;

fn load_config(content: &str) -> Option<GuardrailConfig> {
    toml::from_str(content).ok()
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
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(5),
        Some("does not call `.validate()` on it"),
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn errors_on_explicit_toml_from_str_without_validate() {
    let root = temp_root("rs-garde-14-explicit");
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
use guardrail3_domain_config::types::GuardrailConfig;

fn load_config(content: &str) -> Result<Option<GuardrailConfig>, String> {
    toml::from_str::<GuardrailConfig>(content)
        .map(Some)
        .map_err(|error| error.to_string())
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
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(5),
        Some("`toml::from_str`"),
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn errors_on_try_into_without_validate() {
    let root = temp_root("rs-garde-14-try-into");
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
use guardrail3_domain_config::types::GuardrailConfig;

fn load_config(parsed: toml::Value) -> Option<GuardrailConfig> {
    match parsed.clone().try_into::<GuardrailConfig>() {
        Ok(config) => Some(config),
        Err(_) => None,
    }
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
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(5),
        Some("`try_into::<GuardrailConfig>()`"),
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn errors_when_prior_validate_call_is_on_shadowed_binding_name() {
    let root = temp_root("rs-garde-14-shadowed");
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
use guardrail3_domain_config::types::GuardrailConfig;

struct Wrapper;

impl Wrapper {
    fn validate(&self) -> Result<(), ()> {
        Ok(())
    }
}

fn load_config(cfg: &Wrapper, content: &str) -> Option<GuardrailConfig> {
    cfg.validate().ok()?;
    let cfg = toml::from_str(content).ok()?;
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
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(14),
        Some("does not call `.validate()` on it"),
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}

#[test]
fn errors_when_binding_is_used_before_late_validate() {
    let root = temp_root("rs-garde-14-use-before-validate");
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

fn observe(_cfg: &GuardrailConfig) {}

fn load_config(content: &str) -> Option<GuardrailConfig> {
    let cfg = toml::from_str(content).ok()?;
    observe(&cfg);
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
    assertions::assert_single_error(
        &results,
        Some(source_rel),
        Some(8),
        Some("does not call `.validate()` on it"),
    );

    std::fs::remove_dir_all(root).expect("failed to remove temporary fixture root");
}
