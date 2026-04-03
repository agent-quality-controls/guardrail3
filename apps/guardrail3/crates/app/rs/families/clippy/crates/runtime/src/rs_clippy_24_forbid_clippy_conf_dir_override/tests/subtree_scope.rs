use guardrail3_app_rs_family_clippy_assertions::rs_clippy_24_forbid_clippy_conf_dir_override as assertions;
use test_support::{build_fixture_clippy_toml, dir_entry, project_tree};

use super::helpers::{
    run_family_with_validation_scope_for_tests, run_with_validation_scope_for_tests,
};

#[test]
fn ignores_sibling_override_surfaces_when_validation_scope_targets_one_app() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps"], &["Cargo.toml", "clippy.toml"])),
            ("apps", dir_entry(&["backend", "devctl"], &[])),
            ("apps/backend", dir_entry(&["crates"], &["Cargo.toml"])),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
            (
                "apps/devctl",
                dir_entry(&[".cargo", "crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/devctl/.cargo", dir_entry(&[], &["config.toml"])),
            ("apps/devctl/crates", dir_entry(&["cli"], &[])),
            ("apps/devctl/crates/cli", dir_entry(&[], &["Cargo.toml"])),
        ],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            (
                "clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "apps/devctl/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/devctl/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/devctl/.cargo/config.toml",
                "[env]\nCLIPPY_CONF_DIR = \"..\"\n".to_owned(),
            ),
            (
                "apps/devctl/crates/cli/Cargo.toml",
                "[package]\nname = \"cli\"\n".to_owned(),
            ),
        ],
    );

    let results = run_with_validation_scope_for_tests(&tree, "apps/backend");
    assertions::assert_inventory(&results);
}

#[test]
fn stays_silent_when_validation_scope_contains_no_routed_rust_roots() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps", "docs"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
            ("docs", dir_entry(&["guide"], &[])),
            ("docs/guide", dir_entry(&[".cargo"], &[])),
            ("docs/guide/.cargo", dir_entry(&[], &["config.toml"])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "docs/guide/.cargo/config.toml",
                "[env]\nRUSTFLAGS = \"-Dwarnings\"\n".to_owned(),
            ),
        ],
    );

    let results = run_family_with_validation_scope_for_tests(&tree, "docs/guide");
    assert!(
        results.is_empty(),
        "expected no clippy results outside routed scope: {results:#?}"
    );
}

#[test]
fn stays_silent_when_scoped_override_exists_outside_all_rust_workspaces() {
    let tree = project_tree(
        vec![
            ("", dir_entry(&["apps", "docs"], &[])),
            ("apps", dir_entry(&["backend"], &[])),
            (
                "apps/backend",
                dir_entry(&["crates"], &["Cargo.toml", "clippy.toml"]),
            ),
            ("apps/backend/crates", dir_entry(&["core"], &[])),
            ("apps/backend/crates/core", dir_entry(&[], &["Cargo.toml"])),
            ("docs", dir_entry(&["guide"], &[])),
            ("docs/guide", dir_entry(&[".cargo"], &[])),
            ("docs/guide/.cargo", dir_entry(&[], &["config.toml"])),
        ],
        vec![
            (
                "apps/backend/Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\n".to_owned(),
            ),
            (
                "apps/backend/clippy.toml",
                build_fixture_clippy_toml("service", false, true, "", ""),
            ),
            (
                "apps/backend/crates/core/Cargo.toml",
                "[package]\nname = \"core\"\n".to_owned(),
            ),
            (
                "docs/guide/.cargo/config.toml",
                "[env]\nCLIPPY_CONF_DIR = \".\"\n".to_owned(),
            ),
        ],
    );

    let results = run_family_with_validation_scope_for_tests(&tree, "docs/guide");
    assert!(
        results.is_empty(),
        "expected no clippy results outside routed rust workspaces: {results:#?}"
    );
}
