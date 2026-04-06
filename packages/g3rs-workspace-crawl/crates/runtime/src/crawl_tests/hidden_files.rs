use std::{fs, path::Path};

use g3rs_workspace_crawl_assertions::{
    workspace_entries::{assert_entry, assert_has_rel_path},
    workspace_queries::assert_root_file_exists,
};
use g3rs_workspace_crawl_types::{G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState};
use tempfile::tempdir;

#[test]
fn includes_hidden_config_files() {
    let temp_dir = tempdir().expect("create temporary workspace root");
    let root = temp_dir.path();
    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join("Cargo.toml"), "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n");

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

fn write(path: impl AsRef<Path>, content: &str) {
    fs::write(path, content).expect("write hidden-files fixture file");
}
