use test_support::{dir_entry, project_tree};

use super::collect_for_tests;

#[test]
fn root_config_uses_packages_profile_when_packages_policy_exists() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            ("packages/shared-types", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"packages/shared-types\"]\n".to_owned(),
            ),
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\n[rust.packages.checks]\ngarde = false\n"
                    .to_owned(),
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[package]\nname = \"shared-types\"\n".to_owned(),
            ),
            (
                "clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml("library", true, false, "", ""),
            ),
        ],
    );

    let facts = collect_for_tests(&tree);
    let root = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "clippy.toml")
        .expect("expected root clippy.toml facts");

    assert_eq!(root.profile_name.as_deref(), Some("library"));
    assert!(!root.garde_enabled);
}

#[test]
fn validation_root_clippy_is_collected_without_root_cargo() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["clippy.toml"])),
            ("apps", dir_entry(&["backend"], &[])),
            ("apps/backend", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            (
                "clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
        ],
    );

    let facts = collect_for_tests(&tree);

    assert!(
        facts.allowed_configs.iter().any(|config| config.rel_path == "clippy.toml"),
        "expected validation-root clippy.toml to be collected even when the validation root is not a Rust root: {facts:#?}"
    );
}

#[test]
fn standalone_app_root_uses_rust_apps_profile_policy() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml"])),
            ("apps", dir_entry(&["libsite"], &[])),
            (
                "apps/libsite",
                dir_entry(&[], &["Cargo.toml", "clippy.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"\n[rust.apps.libsite.checks]\ngarde = false\n"
                    .to_owned(),
            ),
            (
                "apps/libsite/Cargo.toml",
                "[package]\nname = \"libsite\"\n".to_owned(),
            ),
            (
                "apps/libsite/clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml("library", false, false, "", ""),
            ),
        ],
    );

    let facts = collect_for_tests(&tree);
    let local = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "apps/libsite/clippy.toml")
        .expect("expected app-local clippy.toml facts");

    assert_eq!(local.profile_name.as_deref(), Some("library"));
    assert!(!local.garde_enabled);
}
