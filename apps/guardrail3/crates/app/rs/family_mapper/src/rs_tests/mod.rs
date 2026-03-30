use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use guardrail3_app_rs_family_mapper_assertions::rs::{assert_disabled, assert_enabled};
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

#[test]
fn other_roots_are_excluded_when_app_scope_is_configured() {
    let config = super::app_scoped_config_test();

    assert_disabled(super::root_enabled_for_toolchain_test(
        &super::root_test("fuzz"),
        Some(&config),
    ));
}

#[test]
fn configured_app_root_stays_enabled_when_app_scope_is_configured() {
    let config = super::app_scoped_config_test();

    assert_enabled(super::root_enabled_for_toolchain_test(
        &super::root_test("apps/guardrail3"),
        Some(&config),
    ));
}

#[test]
fn nested_app_root_stays_enabled_when_app_scope_is_configured() {
    let config = super::app_scoped_config_test();

    assert_enabled(super::root_enabled_for_toolchain_test(
        &super::root_test("tools/apps/guardrail3/crates/api"),
        Some(&config),
    ));
}

#[test]
fn other_roots_follow_global_flag_when_no_scope_is_configured() {
    let config = super::global_toolchain_enabled_config_test();

    assert_enabled(super::root_enabled_for_toolchain_test(
        &super::root_test("fuzz"),
        Some(&config),
    ));
}

#[test]
fn deps_route_preserves_repo_workspace_root_for_enabled_descendant_apps() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned()],
                    vec!["Cargo.toml".to_owned(), "guardrail3.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["api".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/api".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([
            (
                "Cargo.toml".to_owned(),
                "[workspace]\nmembers = [\"apps/*\"]\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                r#"
                    [rust.checks]
                    deps = false

                    [rust.apps.api]
                    profile = "service"

                    [rust.apps.api.checks]
                    deps = true
                "#
                .to_owned(),
            ),
            (
                "apps/api/Cargo.toml".to_owned(),
                "[package]\nname = \"api\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let config = toml::from_str::<guardrail3_domain_config::types::GuardrailConfig>(
        tree.file_content("guardrail3.toml")
            .expect("expected guardrail3.toml"),
    )
    .expect("expected config");
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deps]));
    let route = super::FamilyMapper::new(&tree, &scope, Some(&config), &selected, None)
        .map_rs_deps();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["Cargo.toml", "apps/api/Cargo.toml"]);
}

#[test]
fn cargo_route_validation_scope_excludes_sibling_policy_roots() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(vec!["apps".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned(), "other".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    vec!["src".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend/src".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["lib.rs".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/other".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([
            (
                "apps/backend/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "apps/other/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("apps/backend/src"))
        .map_rs_cargo();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["apps/backend/Cargo.toml"]);
}
