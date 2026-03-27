use super::{build_fixture_deny_toml, collect_for_test, dir_entry, project_tree};

#[test]
fn root_config_uses_packages_profile_when_packages_policy_exists() {
    let tree = project_tree(
        vec![
            (
                "",
                dir_entry(
                    &["packages"],
                    &["Cargo.toml", "guardrail3.toml", "deny.toml"],
                ),
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
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\n".to_owned(),
            ),
            (
                "packages/shared-types/Cargo.toml",
                "[package]\nname = \"shared-types\"\n".to_owned(),
            ),
            (
                "deny.toml",
                build_fixture_deny_toml("library"),
            ),
        ],
    );

    let facts = collect_for_test(&tree);
    let root = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "deny.toml")
        .expect("expected root deny.toml facts");

    assert_eq!(root.profile_name.as_deref(), Some("library"));
}
