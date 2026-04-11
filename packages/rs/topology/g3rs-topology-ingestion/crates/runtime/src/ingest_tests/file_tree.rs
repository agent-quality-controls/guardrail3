use std::fs;

use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyWorkspaceFamily, G3RsTopologyWorkspaceFamilyFileKind,
};
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

#[test]
fn file_tree_ingestion_collects_descendant_roots_and_family_files() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/app", "nested"]
exclude = ["crates/ignored"]
"#,
    );

    fs::create_dir_all(root.path().join("crates/app/src")).expect("app dirs");
    fs::create_dir_all(root.path().join("nested/src")).expect("nested dirs");
    fs::create_dir_all(root.path().join(".cargo")).expect("cargo dir");
    fs::create_dir_all(root.path().join(".config")).expect("config dir");

    write(
        root.path().join("crates/app/Cargo.toml"),
        r#"
[package]
name = "app"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("crates/app/src/lib.rs"),
        "pub struct App;\n",
    );

    write(
        root.path().join("nested/Cargo.toml"),
        r#"
[workspace]
members = []
"#,
    );
    write(
        root.path().join("nested/src/lib.rs"),
        "pub struct Nested;\n",
    );

    write(root.path().join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.path().join(".cargo/config.toml"), "[alias]\n");
    write(
        root.path().join(".cargo/mutants.toml"),
        "timeout_multiplier = 2.0\n",
    );
    write(
        root.path().join(".config/nextest.toml"),
        "[profile.default]\n",
    );
    write(root.path().join("guardrail3-rs.toml"), "[rust]\n");
    write(
        root.path().join("guardrail3.toml"),
        "[profile]\nname = \"service\"\n",
    );

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");

    assert_eq!(input.workspace_root_rel_dir, "");
    assert_eq!(input.workspace_root_cargo_rel_path, "Cargo.toml");
    assert_eq!(
        input
            .workspace_manifest
            .workspace
            .as_ref()
            .expect("workspace")
            .members,
        vec!["crates/app".to_owned(), "nested".to_owned()]
    );

    assert_eq!(input.descendant_cargo_roots.len(), 2);
    assert!(input.descendant_cargo_roots.iter().any(|root| {
        root.rel_dir == "crates/app"
            && root.cargo_rel_path == "crates/app/Cargo.toml"
            && root.manifest_kind == Some(G3RsTopologyCargoManifestKind::Package)
    }));
    assert!(input.descendant_cargo_roots.iter().any(|root| {
        root.rel_dir == "nested"
            && root.cargo_rel_path == "nested/Cargo.toml"
            && root.manifest_kind == Some(G3RsTopologyCargoManifestKind::Workspace)
    }));

    assert!(input.family_files.iter().any(|file| {
        file.family == G3RsTopologyWorkspaceFamily::Clippy
            && file.rel_path == "clippy.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::ClippyToml
    }));
    assert!(input.family_files.iter().any(|file| {
        file.family == G3RsTopologyWorkspaceFamily::Clippy
            && file.rel_path == ".cargo/config.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml
    }));
    assert!(input.family_files.iter().any(|file| {
        file.family == G3RsTopologyWorkspaceFamily::Test
            && file.rel_path == ".cargo/mutants.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::MutantsToml
    }));
    assert!(input.family_files.iter().any(|file| {
        file.family == G3RsTopologyWorkspaceFamily::Test
            && file.rel_path == ".config/nextest.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::NextestToml
    }));
    assert!(input.family_files.iter().any(|file| {
        file.family == G3RsTopologyWorkspaceFamily::Deps
            && file.rel_path == "guardrail3-rs.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml
    }));
    assert!(input.family_files.iter().any(|file| {
        file.family == G3RsTopologyWorkspaceFamily::Garde
            && file.rel_path == "guardrail3.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::GuardrailToml
    }));
    assert!(input.input_failures.is_empty());
}

#[test]
fn unreadable_root_cargo_fails_ingestion() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    let cargo_path = root.path().join("Cargo.toml");
    make_unreadable(&cargo_path);

    let crawl = crawl(root.path()).expect("crawl");
    let err = crate::ingest_for_file_tree_checks(&crawl).expect_err("root cargo should fail");
    restore_readable(&cargo_path);

    assert!(matches!(
        err,
        g3rs_topology_ingestion_types::G3RsTopologyIngestionError::Unreadable { path, .. }
        if path.ends_with("Cargo.toml")
    ));
}

#[test]
fn malformed_root_cargo_fails_ingestion() {
    let root = tempdir().expect("tempdir");

    write(root.path().join("Cargo.toml"), "[workspace");

    let crawl = crawl(root.path()).expect("crawl");
    let err = crate::ingest_for_file_tree_checks(&crawl).expect_err("root cargo should fail");

    assert!(matches!(
        err,
        g3rs_topology_ingestion_types::G3RsTopologyIngestionError::ParseFailed { path, .. }
        if path.ends_with("Cargo.toml")
    ));
}

#[test]
fn malformed_descendant_cargo_becomes_input_failure() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["good", "bad"]
"#,
    );

    fs::create_dir_all(root.path().join("good/src")).expect("good dirs");
    fs::create_dir_all(root.path().join("bad/src")).expect("bad dirs");

    write(
        root.path().join("good/Cargo.toml"),
        r#"
[package]
name = "good"
version = "0.1.0"
"#,
    );
    write(root.path().join("good/src/lib.rs"), "pub struct Good;\n");

    write(root.path().join("bad/Cargo.toml"), "[package");
    write(root.path().join("bad/src/lib.rs"), "pub struct Bad;\n");

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");

    assert!(input.descendant_cargo_roots.iter().any(|root| {
        root.rel_dir == "bad"
            && root.cargo_rel_path == "bad/Cargo.toml"
            && root.manifest_kind.is_none()
    }));
    assert!(
        input
            .input_failures
            .iter()
            .any(|failure| { failure.rel_path == "bad/Cargo.toml" && !failure.message.is_empty() })
    );
}

#[test]
fn unreadable_descendant_cargo_becomes_input_failure() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["good", "bad"]
"#,
    );

    fs::create_dir_all(root.path().join("good/src")).expect("good dirs");
    fs::create_dir_all(root.path().join("bad/src")).expect("bad dirs");

    write(
        root.path().join("good/Cargo.toml"),
        r#"
[package]
name = "good"
version = "0.1.0"
"#,
    );
    write(root.path().join("good/src/lib.rs"), "pub struct Good;\n");

    let bad_cargo = root.path().join("bad/Cargo.toml");
    write(
        bad_cargo.clone(),
        r#"
[package]
name = "bad"
version = "0.1.0"
"#,
    );
    write(root.path().join("bad/src/lib.rs"), "pub struct Bad;\n");
    make_unreadable(&bad_cargo);

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");
    restore_readable(&bad_cargo);

    assert!(input.descendant_cargo_roots.iter().any(|root| {
        root.rel_dir == "bad"
            && root.cargo_rel_path == "bad/Cargo.toml"
            && root.manifest_kind.is_none()
    }));
    assert!(
        input
            .input_failures
            .iter()
            .any(|failure| failure.rel_path == "bad/Cargo.toml")
    );
}

#[test]
fn ignored_descendant_roots_and_family_files_stay_out() {
    let root = tempdir().expect("tempdir");
    git_init(root.path());

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["live"]
"#,
    );
    write(root.path().join(".gitignore"), "ignored/\n");

    fs::create_dir_all(root.path().join("live/src")).expect("live dirs");
    fs::create_dir_all(root.path().join("ignored/src")).expect("ignored dirs");
    fs::create_dir_all(root.path().join("ignored/.cargo")).expect("ignored cargo dir");

    write(
        root.path().join("live/Cargo.toml"),
        r#"
[package]
name = "live"
version = "0.1.0"
"#,
    );
    write(root.path().join("live/src/lib.rs"), "pub struct Live;\n");

    write(
        root.path().join("ignored/Cargo.toml"),
        r#"
[package]
name = "ignored"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("ignored/src/lib.rs"),
        "pub struct Ignored;\n",
    );
    write(root.path().join("ignored/.cargo/config.toml"), "[alias]\n");

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");

    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .all(|root| !root.rel_dir.starts_with("ignored"))
    );
    assert!(
        input
            .family_files
            .iter()
            .all(|file| !file.rel_path.starts_with("ignored/"))
    );
}

#[test]
fn excluded_live_topology_paths_stay_out() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["live"]
"#,
    );

    fs::create_dir_all(root.path().join("live/src")).expect("live dirs");
    fs::create_dir_all(root.path().join("tests/fixtures/demo/src")).expect("fixture dirs");
    fs::create_dir_all(root.path().join("tests/snapshots/demo/src")).expect("snapshot dirs");
    fs::create_dir_all(root.path().join("target/generated/src")).expect("target dirs");
    fs::create_dir_all(root.path().join(".claude/worktrees/tmp/src")).expect("worktree dirs");

    write(
        root.path().join("live/Cargo.toml"),
        r#"
[package]
name = "live"
version = "0.1.0"
"#,
    );
    write(root.path().join("live/src/lib.rs"), "pub struct Live;\n");

    write(
        root.path().join("tests/fixtures/demo/Cargo.toml"),
        r#"
[package]
name = "fixture_demo"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("tests/fixtures/demo/clippy.toml"),
        "msrv = \"1.85\"\n",
    );

    write(
        root.path().join("tests/snapshots/demo/Cargo.toml"),
        r#"
[package]
name = "snapshot_demo"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("tests/snapshots/demo/deny.toml"),
        "advisories = {}\n",
    );

    write(
        root.path().join("target/generated/Cargo.toml"),
        r#"
[package]
name = "target_demo"
version = "0.1.0"
"#,
    );
    write(
        root.path().join(".claude/worktrees/tmp/Cargo.toml"),
        r#"
[package]
name = "worktree_demo"
version = "0.1.0"
"#,
    );

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");

    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .all(|root| !root.rel_dir.starts_with("tests/fixtures/"))
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .all(|root| !root.rel_dir.starts_with("tests/snapshots/"))
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .all(|root| !root.rel_dir.starts_with("target/"))
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .all(|root| !root.rel_dir.starts_with(".claude/worktrees/"))
    );
    assert!(
        input
            .family_files
            .iter()
            .all(|file| !file.rel_path.starts_with("tests/fixtures/"))
    );
    assert!(
        input
            .family_files
            .iter()
            .all(|file| !file.rel_path.starts_with("tests/snapshots/"))
    );
}

#[test]
fn family_file_mapping_covers_supported_workspace_local_files() {
    let root = tempdir().expect("tempdir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    fs::create_dir_all(root.path().join(".cargo")).expect("cargo dir");
    fs::create_dir_all(root.path().join(".config")).expect("config dir");

    write(
        root.path().join("guardrail3.toml"),
        "[profile]\nname = \"service\"\n",
    );
    write(root.path().join("guardrail3-rs.toml"), "[rust]\n");
    write(root.path().join("rustfmt.toml"), "max_width = 100\n");
    write(root.path().join(".rustfmt.toml"), "max_width = 100\n");
    write(
        root.path().join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(root.path().join("rust-toolchain"), "stable\n");
    write(root.path().join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.path().join(".clippy.toml"), "msrv = \"1.85\"\n");
    write(root.path().join(".cargo/config.toml"), "[alias]\n");
    write(root.path().join(".cargo/config"), "[alias]\n");
    write(root.path().join("deny.toml"), "advisories = {}\n");
    write(root.path().join(".deny.toml"), "advisories = {}\n");
    write(root.path().join(".cargo/deny.toml"), "advisories = {}\n");
    write(root.path().join("release-plz.toml"), "[workspace]\n");
    write(root.path().join("cliff.toml"), "[changelog]\n");
    write(
        root.path().join(".cargo/mutants.toml"),
        "timeout_multiplier = 2.0\n",
    );
    write(
        root.path().join(".config/nextest.toml"),
        "[profile.default]\n",
    );

    let crawl = crawl(root.path()).expect("crawl");
    let input = crate::ingest_for_file_tree_checks(&crawl).expect("file-tree ingest");

    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "Cargo.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        "guardrail3.toml",
        G3RsTopologyWorkspaceFamilyFileKind::GuardrailToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deps,
        "guardrail3-rs.toml",
        G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Fmt,
        "rustfmt.toml",
        G3RsTopologyWorkspaceFamilyFileKind::RustfmtToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Fmt,
        ".rustfmt.toml",
        G3RsTopologyWorkspaceFamilyFileKind::DotRustfmtToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "rust-toolchain.toml",
        G3RsTopologyWorkspaceFamilyFileKind::RustToolchainToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "rust-toolchain",
        G3RsTopologyWorkspaceFamilyFileKind::RustToolchainLegacy,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        ".clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyDotToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        ".cargo/config.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        ".cargo/config",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deny,
        "deny.toml",
        G3RsTopologyWorkspaceFamilyFileKind::DenyToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deny,
        ".deny.toml",
        G3RsTopologyWorkspaceFamilyFileKind::DenyDotToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deny,
        ".cargo/deny.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Release,
        "release-plz.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ReleasePlzToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Release,
        "cliff.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CliffToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Test,
        ".cargo/mutants.toml",
        G3RsTopologyWorkspaceFamilyFileKind::MutantsToml,
    );
    assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Test,
        ".config/nextest.toml",
        G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
    );
}

fn write(path: std::path::PathBuf, content: &str) {
    fs::write(path, content).expect("write");
}

fn assert_family_file(
    input: &g3rs_topology_types::G3RsTopologyFileTreeChecksInput,
    family: G3RsTopologyWorkspaceFamily,
    rel_path: &str,
    kind: G3RsTopologyWorkspaceFamilyFileKind,
) {
    assert!(
        input.family_files.iter().any(|file| {
            file.family == family && file.rel_path == rel_path && file.kind == kind
        }),
        "expected family file mapping for {rel_path}"
    );
}

#[cfg(unix)]
fn make_unreadable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o000);
    fs::set_permissions(path, perms).expect("chmod 000");
}

#[cfg(not(unix))]
fn make_unreadable(_path: &std::path::Path) {
    panic!("unreadable test requires unix permissions");
}

#[cfg(unix)]
fn restore_readable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path).expect("metadata").permissions();
    perms.set_mode(0o644);
    fs::set_permissions(path, perms).expect("chmod 644");
}

#[cfg(not(unix))]
fn restore_readable(_path: &std::path::Path) {}

fn git_init(root: &std::path::Path) {
    let status = std::process::Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(root)
        .status()
        .expect("git init");
    assert!(status.success(), "git init should succeed");
}
