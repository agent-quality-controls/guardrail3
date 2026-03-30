use std::collections::BTreeSet;

use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_app_rs_placement::collect as collect_scope;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

use super::{build_fixture_deny_toml, collect, collect_for_test, dir_entry, project_tree};
use test_support::{copy_fixture, walk, write_file};

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
        facts
            .allowed_configs
            .iter()
            .all(|config| !config.policy_context_valid && config.profile_name.is_none()),
        "expected malformed guardrail policy to disable profile-sensitive config evaluation"
    );
}

#[test]
fn invalid_guardrail_policy_shape_fails_closed_for_profile_sensitive_deny_rules() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "deny.toml"]),
        )],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\"\n".to_owned()),
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.apps]\nbackend = \"library\"\n".to_owned(),
            ),
            ("deny.toml", build_fixture_deny_toml("service")),
        ],
    );

    let facts = collect_for_test(&tree);
    let parse_error = facts
        .policy_context_parse_error
        .as_deref()
        .expect("expected deny policy-context shape error");

    assert!(
        parse_error.contains("`rust.apps.backend` must be a table"),
        "expected shape error for invalid rust.apps entry, got `{parse_error}`"
    );
    assert!(
        facts
            .allowed_configs
            .iter()
            .all(|config| !config.policy_context_valid && config.profile_name.is_none()),
        "expected invalid guardrail policy shape to disable profile-sensitive config evaluation"
    );
}

#[test]
fn unknown_guardrail_profile_name_fails_closed_for_profile_sensitive_deny_rules() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "deny.toml"]),
        )],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\"\n".to_owned()),
            ("guardrail3.toml", "[profile]\nname = \"cli\"\n".to_owned()),
            ("deny.toml", build_fixture_deny_toml("service")),
        ],
    );

    let facts = collect_for_test(&tree);
    let parse_error = facts
        .policy_context_parse_error
        .as_deref()
        .expect("expected deny policy-context profile error");

    assert!(
        parse_error.contains("`profile.name` must be `service` or `library`"),
        "expected profile-name error, got `{parse_error}`"
    );
    assert!(
        facts
            .allowed_configs
            .iter()
            .all(|config| !config.policy_context_valid && config.profile_name.is_none()),
        "expected unknown guardrail profile name to disable profile-sensitive config evaluation"
    );
}

#[test]
fn conflicting_type_and_profile_selectors_fail_closed_for_profile_sensitive_deny_rules() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "deny.toml"]),
        )],
        vec![
            ("Cargo.toml", "[package]\nname = \"crate\"\n".to_owned()),
            (
                "guardrail3.toml",
                "[profile]\nname = \"service\"\n[rust.packages]\ntype = \"library\"\nprofile = \"service\"\n".to_owned(),
            ),
            ("deny.toml", build_fixture_deny_toml("library")),
        ],
    );

    let facts = collect_for_test(&tree);
    let parse_error = facts
        .policy_context_parse_error
        .as_deref()
        .expect("expected deny policy-context selector conflict error");

    assert!(
        parse_error.contains("`rust.packages`.type and `rust.packages`.profile must match"),
        "expected type/profile conflict error, got `{parse_error}`"
    );
    assert!(
        facts
            .allowed_configs
            .iter()
            .all(|config| !config.policy_context_valid && config.profile_name.is_none()),
        "expected conflicting type/profile selectors to disable profile-sensitive config evaluation"
    );
}

#[test]
fn scoped_deny_facts_do_not_collect_sibling_root_configs() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/backend/deny.toml",
        &build_fixture_deny_toml("service"),
    );

    let tree = walk(tmp.path());
    let scope = collect_scope(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("apps/devctl"))
        .map_rs_deny();
    let facts = collect(&tree, &route);

    let allowed = facts
        .allowed_configs
        .iter()
        .map(|config| config.rel_path.as_str())
        .collect::<Vec<_>>();
    let forbidden = facts
        .forbidden_configs
        .iter()
        .map(|config| config.rel_path.as_str())
        .collect::<Vec<_>>();
    let covered = facts
        .covered_units
        .iter()
        .map(|unit| unit.rel_dir.as_str())
        .collect::<Vec<_>>();

    assert_eq!(allowed, vec!["apps/devctl/deny.toml", "deny.toml"]);
    assert!(
        forbidden.is_empty(),
        "unexpected forbidden configs: {forbidden:#?}"
    );
    assert_eq!(covered, vec!["", "", "apps/devctl"]);
}

#[test]
fn scoped_deny_facts_still_collect_owned_nested_shadowing_configs() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/crates/domain/types/deny.toml",
        &build_fixture_deny_toml("service"),
    );

    let tree = walk(tmp.path());
    let scope = collect_scope(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("apps/devctl"))
        .map_rs_deny();
    let facts = collect(&tree, &route);

    let forbidden = facts
        .forbidden_configs
        .iter()
        .map(|config| config.rel_path.as_str())
        .collect::<Vec<_>>();

    assert_eq!(forbidden, vec!["apps/devctl/crates/domain/types/deny.toml"]);
}

#[test]
fn validation_root_deny_config_still_covers_routed_roots_without_root_cargo() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["packages"], &["deny.toml"])),
            ("packages", dir_entry(&["shared-types"], &[])),
            ("packages/shared-types", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("deny.toml", build_fixture_deny_toml("library")),
            (
                "packages/shared-types/Cargo.toml",
                "[package]\nname = \"shared-types\"\n".to_owned(),
            ),
        ],
    );

    let scope = collect_scope(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deny();
    let facts = collect(&tree, &route);

    let allowed = facts
        .allowed_configs
        .iter()
        .map(|config| config.rel_path.as_str())
        .collect::<Vec<_>>();
    let covered = facts
        .covered_units
        .iter()
        .map(|unit| (unit.rel_dir.as_str(), unit.covering_config_rel.as_str()))
        .collect::<Vec<_>>();

    assert_eq!(allowed, vec!["deny.toml"]);
    assert_eq!(
        covered,
        vec![("", "deny.toml"), ("packages/shared-types", "deny.toml")]
    );
    assert!(facts.uncovered_units.is_empty());
}
