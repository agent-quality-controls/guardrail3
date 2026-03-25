use super::super::test_support::{dir_entry, project_tree};
use super::collect;

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

    let facts = collect(&tree);
    let root = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "clippy.toml")
        .expect("expected root clippy.toml facts");

    assert_eq!(root.profile_name.as_deref(), Some("library"));
    assert!(!root.garde_enabled);
}
