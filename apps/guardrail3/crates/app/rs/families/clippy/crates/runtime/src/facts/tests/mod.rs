use guardrail3_app_rs_family_clippy_assertions::facts as assertions;
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
    assertions::assert_root_config_uses_packages_profile_when_packages_policy_exists(
        root.profile_name.as_deref(),
        root.garde_enabled,
    );
}

#[test]
fn workspace_local_app_root_uses_rust_apps_profile_policy() {
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
                "[workspace]\nmembers = []\n[package]\nname = \"libsite\"\n".to_owned(),
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
    assertions::assert_workspace_local_app_root_uses_rust_apps_profile_policy(
        local.profile_name.as_deref(),
        local.garde_enabled,
    );
}

#[test]
fn malformed_guardrail_policy_is_recorded_as_policy_context_error() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            ("guardrail3.toml", "[".to_owned()),
            (
                "clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml(
                    "service", false, true, "", "",
                ),
            ),
        ],
    );

    let facts = collect_for_tests(&tree);
    assertions::assert_malformed_guardrail_policy_is_recorded_as_policy_context_error(
        facts
            .policy_context_parse_error
            .as_deref()
            .is_some_and(|message| message.contains("TOML parse error")),
        facts
            .allowed_configs
            .iter()
            .all(|config| config.policy_context_parse_error.is_some()),
    );
}

#[test]
fn package_workspace_root_uses_rust_packages_profile_policy() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(&["packages"], &["guardrail3.toml", "clippy.toml"]),
            ),
            ("packages", dir_entry(&["shared-types"], &[])),
            (
                "packages/shared-types",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("packages/shared-types/crates", dir_entry(&["core"], &[])),
            (
                "packages/shared-types/crates/core",
                dir_entry(&[], &["Cargo.toml"]),
            ),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\n[rust.packages.checks]\ngarde = false\n"
                    .to_owned(),
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "packages/shared-types/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml("service", false, true, "", ""),
            ),
            (
                "packages/shared-types/clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml("library", false, false, "", ""),
            ),
        ],
    );

    let facts = collect_for_tests(&tree);
    let local = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "packages/shared-types/clippy.toml")
        .expect("expected package-workspace clippy.toml facts");
    assertions::assert_package_workspace_root_uses_rust_packages_profile_policy(
        local.profile_name.as_deref(),
        local.garde_enabled,
    );
}
