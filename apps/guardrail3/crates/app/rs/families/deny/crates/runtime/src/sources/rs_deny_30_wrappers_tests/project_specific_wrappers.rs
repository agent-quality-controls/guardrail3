use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_30_wrappers as assertions;

use super::super::{add_deny_ban_entry, build_fixture_deny_toml, set_deny_ban_wrappers};

#[test]
fn inventories_added_wrappers_for_bans_without_managed_wrapper_policy() {
    let results = super::super::run_check(&set_deny_ban_wrappers(
        &build_fixture_deny_toml("service"),
        "anyhow",
        &["texting_robots"],
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "project-specific ban wrappers",
            "`deny.toml` ban `anyhow` adds project-specific wrappers `texting_robots`.",
            "deny.toml",
            true,
        )],
    );
}

#[test]
fn inventories_project_specific_wrappers_for_non_canonical_bans() {
    let results = super::super::run_check(&add_deny_ban_entry(
        &build_fixture_deny_toml("service"),
        toml::Value::Table(toml::map::Map::from_iter([
            (
                "name".to_owned(),
                toml::Value::String("custom-crate".to_owned()),
            ),
            (
                "wrappers".to_owned(),
                toml::Value::Array(vec![toml::Value::String("adapter".to_owned())]),
            ),
            (
                "reason".to_owned(),
                toml::Value::String("good enough reason text".to_owned()),
            ),
        ])),
    ));
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "project-specific ban wrappers",
            "`deny.toml` ban `custom-crate` adds project-specific wrappers `adapter`.",
            "deny.toml",
            true,
        )],
    );
}
