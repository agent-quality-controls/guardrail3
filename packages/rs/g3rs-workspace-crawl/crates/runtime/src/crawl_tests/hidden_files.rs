use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_workspace_crawl_assertions::{
    workspace_entries::{assert_entry, assert_has_rel_path},
    workspace_queries::assert_root_file_exists,
};
use g3rs_workspace_crawl_types::{G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState};
use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

#[test]
fn includes_hidden_config_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    git_init(root);
    fs::write(root.join(".clippy.toml"), "msrv = \"1.85\"\n").expect("write hidden config fixture");
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write Cargo.toml fixture");

    let crawl = crate::crawl(root).expect("crawl should succeed");

    assert_root_file_exists(&crawl, ".clippy.toml");
    assert_has_rel_path(&crawl.entries, "Cargo.toml");
    assert_entry(
        crawl.entry(".clippy.toml").expect("hidden config entry"),
        G3RsWorkspaceEntryKind::File,
        G3RsWorkspaceIgnoreState::Included,
        true,
    );
}
