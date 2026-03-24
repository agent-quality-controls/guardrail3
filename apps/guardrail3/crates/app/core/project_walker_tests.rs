use std::fs;

use tempfile::tempdir;

use super::walk_project;
use crate::adapters::outbound::fs::RealFileSystem;

#[test]
fn preserves_immediate_ignored_file_children_in_discovered_dirs() {
    let tmp = tempdir().expect("tempdir");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/core")).expect("create dirs");
    fs::write(tmp.path().join(".gitignore"), "*.env\n").expect("write gitignore");
    fs::write(tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n").expect("write cargo");
    fs::write(tmp.path().join("apps/devctl/crates/app/.env"), "SECRET=1").expect("write env");
    fs::write(
        tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("write lib");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.files.iter().any(|file| file == ".env"),
        "ignored immediate file should still be visible in ProjectTree: {entry:#?}"
    );
}

#[test]
fn preserves_immediate_ignored_directory_children_in_discovered_dirs() {
    let tmp = tempdir().expect("tempdir");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/core")).expect("create dirs");
    fs::write(tmp.path().join(".gitignore"), "orphan/\n").expect("write gitignore");
    fs::write(tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n").expect("write cargo");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/orphan/src")).expect("mkdir");
    fs::write(
        tmp.path().join("apps/devctl/crates/app/orphan/src/lib.rs"),
        "pub fn ignored_leaf() {}\n",
    )
    .expect("write lib");
    fs::write(
        tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("write lib");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.dirs.iter().any(|dir| dir == "orphan")
            && tree.dir_exists("apps/devctl/crates/app/orphan"),
        "ignored immediate child directory should still be visible in ProjectTree: {entry:#?}"
    );
}

#[test]
fn recursively_scans_newly_recovered_ignored_directories() {
    let tmp = tempdir().expect("tempdir");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/core")).expect("create dirs");
    fs::write(tmp.path().join(".gitignore"), "valid_crate/\n").expect("write gitignore");
    fs::write(tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n").expect("write cargo");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/valid_crate")).expect("mkdir");
    fs::write(
        tmp.path()
            .join("apps/devctl/crates/app/valid_crate/Cargo.toml"),
        "[package]\nname = \"valid-crate\"\nversion = \"0.1.0\"\n",
    )
    .expect("write cargo");
    fs::write(
        tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("write lib");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app/valid_crate")
        .expect("recovered leaf dir discovered");

    assert!(
        entry.files.iter().any(|file| file == "Cargo.toml"),
        "recovered ignored directories should also have their immediate children scanned: {entry:#?}"
    );
}

#[test]
#[cfg(unix)]
fn preserves_immediate_ignored_symlink_file_children_in_discovered_dirs() {
    let tmp = tempdir().expect("tempdir");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/core")).expect("create dirs");
    fs::write(tmp.path().join(".gitignore"), "*.env\n").expect("write gitignore");
    fs::write(tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n").expect("write cargo");
    std::os::unix::fs::symlink(
        tmp.path().join("apps/devctl/Cargo.toml"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("symlink");
    fs::write(
        tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("write lib");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.files.iter().any(|file| file == ".env")
            && entry.symlink_files.iter().any(|file| file == ".env"),
        "ignored immediate symlink file should stay visible as a symlink child: {entry:#?}"
    );
}

#[test]
#[cfg(unix)]
fn preserves_immediate_broken_symlink_children_in_discovered_dirs() {
    let tmp = tempdir().expect("tempdir");
    fs::create_dir_all(tmp.path().join("apps/devctl/crates/app/core")).expect("create dirs");
    fs::write(tmp.path().join("apps/devctl/Cargo.toml"), "[workspace]\n").expect("write cargo");
    std::os::unix::fs::symlink(
        tmp.path().join("missing-target"),
        tmp.path().join("apps/devctl/crates/app/.env"),
    )
    .expect("symlink");
    fs::write(
        tmp.path().join("apps/devctl/crates/app/core/lib.rs"),
        "pub fn ready() {}\n",
    )
    .expect("write lib");

    let tree = walk_project(&RealFileSystem, tmp.path());
    let entry = tree
        .dir_contents("apps/devctl/crates/app")
        .expect("app container discovered");

    assert!(
        entry.files.iter().any(|file| file == ".env")
            && entry.symlink_files.iter().any(|file| file == ".env"),
        "broken immediate symlink child should stay visible as a symlink file: {entry:#?}"
    );
}
