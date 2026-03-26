use std::collections::BTreeSet;

use super::super::discovered_app_rel_dirs_for_tests;
use super::{copy_fixture, create_dir, walk, write_file};

fn discovered_apps(root: &std::path::Path) -> BTreeSet<String> {
    let _ = walk(root);
    discovered_app_rel_dirs_for_tests(root)
}

#[test]
fn newly_discovered_rust_app_without_crates_is_collected_as_an_app_root() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom");
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let apps = discovered_apps(tmp.path());
    assert!(apps.contains("apps/phantom"), "{apps:#?}");
}

#[test]
fn newly_discovered_app_with_empty_cargo_toml_is_still_collected_as_an_app_root() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom");
    write_file(tmp.path(), "apps/phantom/Cargo.toml", "");

    let apps = discovered_apps(tmp.path());
    assert!(apps.contains("apps/phantom"), "{apps:#?}");
}

#[test]
fn newly_discovered_app_with_malformed_cargo_toml_is_still_collected_as_an_app_root() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/phantom");
    write_file(
        tmp.path(),
        "apps/phantom/Cargo.toml",
        "this is not valid toml {{{{\n",
    );

    let apps = discovered_apps(tmp.path());
    assert!(apps.contains("apps/phantom"), "{apps:#?}");
}

#[test]
fn cargo_toml_directory_is_not_collected_as_an_app_root() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/broken/Cargo.toml");

    let apps = discovered_apps(tmp.path());
    assert!(!apps.contains("apps/broken"), "{apps:#?}");
}

#[test]
fn broken_cargo_toml_symlink_is_not_collected_as_an_app_root() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/broken");
    std::os::unix::fs::symlink("/nonexistent", tmp.path().join("apps/broken/Cargo.toml"))
        .expect("symlink");

    let apps = discovered_apps(tmp.path());
    assert!(!apps.contains("apps/broken"), "{apps:#?}");
}

#[test]
fn newly_discovered_extra_app_name_is_collected_as_an_app_root() {
    let tmp = copy_fixture();
    create_dir(tmp.path(), "apps/uber-service");
    write_file(
        tmp.path(),
        "apps/uber-service/Cargo.toml",
        "[workspace]\nmembers = []\n",
    );

    let apps = discovered_apps(tmp.path());
    assert!(apps.contains("apps/uber-service"), "{apps:#?}");
}
