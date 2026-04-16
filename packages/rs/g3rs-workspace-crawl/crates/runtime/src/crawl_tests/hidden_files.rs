use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_workspace_crawl_assertions::crawl as assertions;
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

    assertions::assert_root_file_exists(&crawl, ".clippy.toml");
    assertions::assert_has_rel_path(&crawl.entries, "Cargo.toml");
    assertions::assert_crawl_entry(
        &crawl,
        ".clippy.toml",
        crate::G3RsWorkspaceEntryKind::File,
        crate::G3RsWorkspaceIgnoreState::Included,
        true,
    );
}
