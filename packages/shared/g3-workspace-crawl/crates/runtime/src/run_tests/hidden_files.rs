#![allow(
    clippy::disallowed_methods,
    reason = "fixture-driven filesystem tests need direct std::fs calls in test bodies"
)]

use std::fs;

use g3_workspace_crawl_assertions::run as assertions;
use tempfile::tempdir;

use super::helpers::git_init;

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

    let crawl = crate::run::crawl(root).expect("crawl should succeed");

    assertions::assert_root_file_exists(&crawl, ".clippy.toml");
    assertions::assert_has_rel_path(&crawl.entries, "Cargo.toml");
    assertions::assert_crawl_entry(
        &crawl,
        ".clippy.toml",
        crate::G3WorkspaceEntryKind::File,
        crate::G3WorkspaceIgnoreState::Included,
        true,
    );
}
