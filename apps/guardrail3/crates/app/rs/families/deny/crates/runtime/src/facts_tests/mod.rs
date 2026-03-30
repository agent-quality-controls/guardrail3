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
            ("deny.toml", build_fixture_deny_toml("library")),
        ],
    );

    let facts = collect_for_test(&tree);
    let root = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "deny.toml")
        .expect("expected root deny.toml facts");

    assert_eq!(root.profile_name.as_deref(), Some("library"));
    assert!(root.policy_context_valid);
}

#[test]
fn standalone_app_root_uses_rust_apps_profile_policy() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["guardrail3.toml", "deny.toml"])),
            ("apps", dir_entry(&["libsite"], &[])),
            ("apps/libsite", dir_entry(&[], &["Cargo.toml", "deny.toml"])),
        ],
        vec![
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps.libsite]\ntype = \"library\"\n"
                    .to_owned(),
            ),
            ("deny.toml", build_fixture_deny_toml("service")),
            (
                "apps/libsite/Cargo.toml",
                "[package]\nname = \"libsite\"\n".to_owned(),
            ),
            ("apps/libsite/deny.toml", build_fixture_deny_toml("library")),
        ],
    );

    let facts = collect_for_test(&tree);
    let local = facts
        .allowed_configs
        .iter()
        .find(|config| config.rel_path == "apps/libsite/deny.toml")
        .expect("expected standalone app deny.toml facts");

    assert_eq!(local.profile_name.as_deref(), Some("library"));
    assert!(local.policy_context_valid);
}

#[test]
fn malformed_guardrail_policy_is_recorded_as_policy_context_error() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "deny.toml"]),
        )],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\"\n".to_owned()),
            ("guardrail3.toml", "[".to_owned()),
            ("deny.toml", build_fixture_deny_toml("library")),
        ],
    );

    let facts = collect_for_test(&tree);
    let parse_error = facts
        .policy_context_parse_error
        .as_deref()
        .expect("expected deny policy-context parse error");
    assert!(
        parse_error.contains("TOML parse error"),
        "expected parse error to contain TOML parse error, got `{parse_error}`"
    );
    assert!(
        facts.allowed_configs
            .iter()
            .all(|config| !config.policy_context_valid && config.profile_name.is_none()),
        "expected malformed guardrail policy to disable profile-sensitive config evaluation"
    );
}
