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
fn deps_route_drops_repo_workspace_root_when_enabled_descendant_app_is_not_a_workspace() {
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
                DirEntry::new(vec!["api".to_owned()], Vec::new(), Vec::new(), Vec::new()),
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
    let route =
        super::FamilyMapper::new(&tree, &scope, Some(&config), &selected, None).map_rs_deps();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["apps/api/Cargo.toml"]);
}

#[test]
fn toolchain_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "rust-toolchain.toml"],
        &[("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\n")],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Toolchain]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_toolchain();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert!(route
        .family_files()
        .iter()
        .any(|file| file.rel_path() == "tools/helper/rust-toolchain.toml"));
}

#[test]
fn clippy_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "clippy.toml"],
        &[
            ("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\n"),
            ("tools/helper/clippy.toml", "msrv = \"1.85\"\n"),
        ],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Clippy]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_clippy();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert_eq!(
        route.family_files()
            .iter()
            .map(|file| file.rel_path().to_owned())
            .collect::<Vec<_>>(),
        vec![
            "tools/helper/Cargo.toml".to_owned(),
            "tools/helper/clippy.toml".to_owned(),
        ]
    );
}

#[test]
fn deny_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "deny.toml"],
        &[
            ("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\n"),
            ("tools/helper/deny.toml", "[bans]\nmultiple-versions = \"deny\"\n"),
        ],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deny]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deny();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert_eq!(
        route.family_files()
            .iter()
            .map(|file| file.rel_path().to_owned())
            .collect::<Vec<_>>(),
        vec![
            "tools/helper/Cargo.toml".to_owned(),
            "tools/helper/deny.toml".to_owned(),
        ]
    );
}

#[test]
fn cargo_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "guardrail3.toml"],
        &[
            ("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\n"),
            ("tools/helper/guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_cargo();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert_eq!(
        route.family_files()
            .iter()
            .map(|file| file.rel_path().to_owned())
            .collect::<Vec<_>>(),
        vec![
            "tools/helper/Cargo.toml".to_owned(),
            "tools/helper/guardrail3.toml".to_owned(),
        ]
    );
}

#[test]
fn deps_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "guardrail3.toml"],
        &[
            ("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\n"),
            ("tools/helper/guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deps]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deps();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert_eq!(
        route.family_files()
            .iter()
            .map(|file| file.rel_path().to_owned())
            .collect::<Vec<_>>(),
        vec![
            "tools/helper/Cargo.toml".to_owned(),
            "tools/helper/guardrail3.toml".to_owned(),
        ]
    );
}

#[test]
fn garde_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "clippy.toml", "guardrail3.toml"],
        &[
            ("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\n"),
            ("tools/helper/clippy.toml", "msrv = \"1.85\"\n"),
            ("tools/helper/guardrail3.toml", "[profile]\nname = \"service\"\n"),
        ],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Garde]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_garde();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.root().cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert_eq!(
        route.family_files()
            .iter()
            .map(|file| file.rel_path().to_owned())
            .collect::<Vec<_>>(),
        vec![
            "tools/helper/Cargo.toml".to_owned(),
            "tools/helper/clippy.toml".to_owned(),
            "tools/helper/guardrail3.toml".to_owned(),
        ]
    );
}

#[test]
fn release_route_keeps_non_workspace_roots_visible_for_family_judgment() {
    let tree = single_other_root_tree(
        &["Cargo.toml", "release-plz.toml"],
        &[
            ("tools/helper/Cargo.toml", "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n"),
            ("tools/helper/release-plz.toml", "[workspace]\n"),
        ],
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Release]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_release();

    let cargo_paths = route
        .roots()
        .iter()
        .map(|root| root.cargo_rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(cargo_paths, vec!["tools/helper/Cargo.toml".to_owned()]);
    assert_eq!(
        route.family_files()
            .iter()
            .map(|file| file.rel_path().to_owned())
            .collect::<Vec<_>>(),
        vec![
            "tools/helper/Cargo.toml".to_owned(),
            "tools/helper/release-plz.toml".to_owned(),
        ]
    );
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

#[test]
fn cargo_route_scope_keeps_descendant_member_manifests_of_the_routed_workspace() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["crates".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "crates".to_owned(),
                DirEntry::new(
                    vec!["api".to_owned(), "other".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "crates/api".to_owned(),
                DirEntry::new(
                    vec!["src".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "crates/api/src".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["lib.rs".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "crates/other".to_owned(),
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
                "[workspace]\nmembers = [\"crates/api\", \"crates/other\"]\nresolver = \"2\"\n"
                    .to_owned(),
            ),
            (
                "crates/api/Cargo.toml".to_owned(),
                "[package]\nname = \"api\"\n".to_owned(),
            ),
            (
                "crates/other/Cargo.toml".to_owned(),
                "[package]\nname = \"other\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("crates/api/src"))
        .map_rs_cargo();

    let cargo_files = route
        .family_files()
        .iter()
        .filter(|file| file.kind() == guardrail3_app_rs_ownership::RustFamilyFileKind::CargoToml)
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        cargo_files,
        vec![
            "Cargo.toml".to_owned(),
            "crates/api/Cargo.toml".to_owned(),
        ]
    );
}

#[test]
fn toolchain_route_keeps_rootless_and_ancestor_toolchain_files_visible() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned(), "tools".to_owned()],
                    vec!["rust-toolchain.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["rust-toolchain".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([(
            "apps/backend/Cargo.toml".to_owned(),
            "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
        )]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Toolchain]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_toolchain();

    let toolchain_files = route
        .family_files()
        .iter()
        .map(|file| {
            (
                file.rel_path().to_owned(),
                file.logical_owner_rel().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        toolchain_files,
        vec![
            ("apps/backend/Cargo.toml".to_owned(), "apps/backend".to_owned()),
            ("rust-toolchain.toml".to_owned(), "".to_owned()),
            (
                "tools/helper/rust-toolchain".to_owned(),
                "tools/helper".to_owned()
            ),
        ]
    );
}

#[test]
fn toolchain_route_drops_outside_root_candidates_when_scope_does_not_touch_them() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned(), "tools".to_owned()],
                    vec!["rust-toolchain.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
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
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["rust-toolchain".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([(
            "apps/backend/Cargo.toml".to_owned(),
            "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
        )]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Toolchain]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("apps/backend/src"))
        .map_rs_toolchain();

    let toolchain_files = route
        .family_files()
        .iter()
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        toolchain_files,
        vec![
            "apps/backend/Cargo.toml".to_owned(),
            "rust-toolchain.toml".to_owned(),
        ]
    );
}

#[test]
fn clippy_route_keeps_only_scope_relevant_candidates_when_scope_is_narrow() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned()],
                    vec!["clippy.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned(), "devctl".to_owned()],
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
                "apps/devctl".to_owned(),
                DirEntry::new(
                    vec![".cargo".to_owned()],
                    vec!["Cargo.toml".to_owned(), "clippy.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/devctl/.cargo".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["config.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([
            ("clippy.toml".to_owned(), "msrv = \"1.85\"\n".to_owned()),
            (
                "apps/backend/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "apps/devctl/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
            (
                "apps/devctl/clippy.toml".to_owned(),
                "msrv = \"1.85\"\n".to_owned(),
            ),
            (
                "apps/devctl/.cargo/config.toml".to_owned(),
                "[env]\nCLIPPY_CONF_DIR = \".\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Clippy]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("apps/backend/src"))
        .map_rs_clippy();

    let files = route
        .family_files()
        .iter()
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec!["apps/backend/Cargo.toml".to_owned(), "clippy.toml".to_owned()]
    );
}

#[test]
fn clippy_route_keeps_ancestor_cargo_override_when_scope_targets_descendant_workspace() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec![".cargo".to_owned(), "apps".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                ".cargo".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["config.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
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
        ]),
        BTreeMap::from([
            (
                ".cargo/config.toml".to_owned(),
                "[env]\nCLIPPY_CONF_DIR = \".\"\n".to_owned(),
            ),
            (
                "apps/backend/Cargo.toml".to_owned(),
                "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Clippy]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None)
        .with_validation_scope(Some("apps/backend/src"))
        .map_rs_clippy();

    let files = route
        .family_files()
        .iter()
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec![
            ".cargo/config.toml".to_owned(),
            "apps/backend/Cargo.toml".to_owned()
        ]
    );
}

#[test]
fn clippy_route_keeps_outside_root_candidates_visible_in_full_tree_runs() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned(), "tools".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["clippy.toml".to_owned()],
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
                "tools/helper/clippy.toml".to_owned(),
                "msrv = \"1.85\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Clippy]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_clippy();

    let files = route
        .family_files()
        .iter()
        .map(|file| {
            (
                file.rel_path().to_owned(),
                file.logical_owner_rel().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec![
            ("apps/backend/Cargo.toml".to_owned(), "apps/backend".to_owned()),
            (
                "tools/helper/clippy.toml".to_owned(),
                "tools/helper".to_owned()
            )
        ]
    );
}

#[test]
fn deny_route_normalizes_cargo_deny_owner_to_parent_root() {
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
                    vec!["backend".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    vec![".cargo".to_owned()],
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend/.cargo".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["deny.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        BTreeMap::from([(
            "apps/backend/Cargo.toml".to_owned(),
            "[workspace]\nmembers = []\nresolver = \"2\"\n".to_owned(),
        )]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deny]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deny();

    let file = route
        .family_files()
        .iter()
        .find(|file| file.rel_path() == "apps/backend/.cargo/deny.toml")
        .expect("expected cargo deny route file");
    assert_eq!(file.logical_owner_rel(), "apps/backend");
    assert_eq!(file.nearest_rust_root_rel(), Some("apps/backend"));
}

#[test]
fn deny_route_keeps_outside_root_candidates_visible_in_full_tree_runs() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned(), "tools".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
                DirEntry::new(
                    vec![".cargo".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper/.cargo".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["deny.toml".to_owned()],
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
                "tools/helper/.cargo/deny.toml".to_owned(),
                "[bans]\nmultiple-versions = \"deny\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deny]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deny();

    let files = route
        .family_files()
        .iter()
        .map(|file| {
            (
                file.rel_path().to_owned(),
                file.logical_owner_rel().to_owned(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec![
            ("apps/backend/Cargo.toml".to_owned(), "apps/backend".to_owned()),
            (
                "tools/helper/.cargo/deny.toml".to_owned(),
                "tools/helper".to_owned()
            )
        ]
    );
}

#[test]
fn cargo_route_keeps_outside_root_candidates_visible_in_full_tree_runs() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned(), "tools".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    vec!["guardrail3.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
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
                "tools/helper/Cargo.toml".to_owned(),
                "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "tools/guardrail3.toml".to_owned(),
                "[profile]\nname = \"library\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Cargo]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_cargo();

    let files = route
        .family_files()
        .iter()
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec![
            "apps/backend/Cargo.toml".to_owned(),
            "tools/guardrail3.toml".to_owned(),
            "tools/helper/Cargo.toml".to_owned(),
        ]
    );
}

#[test]
fn deps_route_keeps_outside_root_candidates_visible_in_full_tree_runs() {
    let tree = ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(
                    vec!["apps".to_owned(), "tools".to_owned()],
                    vec!["guardrail3.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps".to_owned(),
                DirEntry::new(
                    vec!["backend".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "apps/backend".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    vec!["Cargo.toml".to_owned()],
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
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
                "tools/helper/Cargo.toml".to_owned(),
                "[package]\nname = \"helper\"\nversion = \"0.1.0\"\n".to_owned(),
            ),
            (
                "guardrail3.toml".to_owned(),
                "[profile]\nname = \"service\"\n".to_owned(),
            ),
        ]),
    );
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Deps]));
    let route = super::FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_deps();

    let files = route
        .family_files()
        .iter()
        .map(|file| file.rel_path().to_owned())
        .collect::<Vec<_>>();
    assert_eq!(
        files,
        vec![
            "apps/backend/Cargo.toml".to_owned(),
            "guardrail3.toml".to_owned(),
            "tools/helper/Cargo.toml".to_owned(),
        ]
    );
}

fn single_other_root_tree(
    root_files: &[&str],
    content: &[(&str, &str)],
) -> ProjectTree {
    ProjectTree::new(
        PathBuf::from("/tmp/project"),
        BTreeMap::from([
            (
                "".to_owned(),
                DirEntry::new(vec!["tools".to_owned()], Vec::new(), Vec::new(), Vec::new()),
            ),
            (
                "tools".to_owned(),
                DirEntry::new(
                    vec!["helper".to_owned()],
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
            (
                "tools/helper".to_owned(),
                DirEntry::new(
                    Vec::new(),
                    root_files.iter().map(|file| (*file).to_owned()).collect(),
                    Vec::new(),
                    Vec::new(),
                ),
            ),
        ]),
        content
            .iter()
            .map(|(path, value)| ((*path).to_owned(), (*value).to_owned()))
            .collect(),
    )
}
