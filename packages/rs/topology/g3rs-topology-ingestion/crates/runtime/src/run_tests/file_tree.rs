#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures must use std::fs and std::process::Command directly to seed and tear down filesystem state"
)]
#![expect(
    clippy::panic,
    reason = "test fixtures may panic when setup invariants fail"
)]
#![expect(
    clippy::shadow_unrelated,
    reason = "test fixture code reuses bindings like `root` for staged setup steps"
)]
use std::fs;

use g3rs_topology_ingestion_assertions::run as assertions;
use g3rs_topology_types::{
    G3RsTopologyCargoManifestKind, G3RsTopologyWorkspaceFamily,
    G3RsTopologyWorkspaceFamilyFileAttachment, G3RsTopologyWorkspaceFamilyFileKind,
};
use g3rs_workspace_crawl::crawl;
use tempfile::tempdir;

#[test]
fn file_tree_ingestion_collects_descendant_roots_and_family_files() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/app", "nested"]
exclude = ["crates/ignored"]
"#,
    );

    fs::create_dir_all(root.path().join("crates/app/src")).expect("create app source dir");
    fs::create_dir_all(root.path().join("nested/src")).expect("create nested source dir");
    fs::create_dir_all(root.path().join(".cargo")).expect("create cargo config dir");
    fs::create_dir_all(root.path().join(".config")).expect("create config dir");

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
        r"
[workspace]
members = []
",
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
    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assert_eq!(input.workspace_root_rel_dir, "");
    assert_eq!(input.workspace_root_cargo_rel_path, "Cargo.toml");
    assert_eq!(
        input
            .workspace_manifest
            .workspace
            .as_ref()
            .expect("workspace manifest should parse as workspace")
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
            && file.rel_path == "guardrail3-rs.toml"
            && file.kind == G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml
    }));
    assert!(
        !input
            .family_files
            .iter()
            .any(|file| { file.rel_path == "guardrail3.toml" })
    );
    assertions::assert_no_input_failures(&input);
}

#[test]
fn unreadable_root_cargo_fails_ingestion() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    let cargo_path = root.path().join("Cargo.toml");
    make_unreadable(&cargo_path);

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let err = super::super::ingest_for_file_tree_checks(&crawl)
        .expect_err("root manifest should fail ingestion");
    restore_readable(&cargo_path);

    match err {
        g3rs_topology_ingestion_types::G3RsTopologyIngestionError::Unreadable { path, reason } => {
            assert!(path.ends_with("Cargo.toml"));
            assert_eq!(reason, "file is not readable");
        }
        other => panic!("unexpected ingestion error: {other:?}"),
    }
}

#[test]
fn malformed_root_cargo_fails_ingestion() {
    let root = tempdir().expect("create temp dir");

    write(root.path().join("Cargo.toml"), "[workspace");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let err = super::super::ingest_for_file_tree_checks(&crawl)
        .expect_err("malformed root manifest should fail ingestion");

    match err {
        g3rs_topology_ingestion_types::G3RsTopologyIngestionError::ParseFailed { path, reason } => {
            assert!(path.ends_with("Cargo.toml"));
            assert!(reason.contains("invalid table header"), "{reason}");
        }
        other => panic!("unexpected ingestion error: {other:?}"),
    }
}

#[test]
fn non_workspace_root_cargo_fails_ingestion() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[package]
name = "demo"
version = "0.1.0"
"#,
    );

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let err = super::super::ingest_for_file_tree_checks(&crawl)
        .expect_err("non-workspace root manifest should fail ingestion");

    assert!(matches!(
        err,
        g3rs_topology_ingestion_types::G3RsTopologyIngestionError::RootManifestNotWorkspace { path }
        if path.ends_with("Cargo.toml")
    ));
}

#[test]
fn malformed_descendant_cargo_becomes_input_failure() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["good", "bad"]
"#,
    );

    fs::create_dir_all(root.path().join("good/src")).expect("create good source dir");
    fs::create_dir_all(root.path().join("bad/src")).expect("create bad source dir");

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

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_descendant_root(&input, "bad", "bad/Cargo.toml", None);
    assertions::assert_input_failure_contains(&input, "bad/Cargo.toml", "invalid table header");
}

#[test]
fn unreadable_descendant_cargo_becomes_input_failure() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["good", "bad"]
"#,
    );

    fs::create_dir_all(root.path().join("good/src")).expect("create good source dir");
    fs::create_dir_all(root.path().join("bad/src")).expect("create bad source dir");

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

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");
    restore_readable(&bad_cargo);

    assertions::assert_descendant_root(&input, "bad", "bad/Cargo.toml", None);
    assertions::assert_input_failure_contains(&input, "bad/Cargo.toml", "file is not readable");
}

#[test]
fn hybrid_descendant_manifest_is_classified_as_hybrid() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["hybrid"]
"#,
    );

    fs::create_dir_all(root.path().join("hybrid/src")).expect("create hybrid source dir");
    write(
        root.path().join("hybrid/Cargo.toml"),
        r#"
[package]
name = "hybrid"
version = "0.1.0"

[workspace]
members = []
"#,
    );
    write(
        root.path().join("hybrid/src/lib.rs"),
        "pub struct Hybrid;\n",
    );

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assert!(input.descendant_cargo_roots.iter().any(|root| {
        root.rel_dir == "hybrid"
            && root.manifest_kind == Some(G3RsTopologyCargoManifestKind::Hybrid)
    }));
}

#[test]
fn descendant_manifest_with_no_workspace_or_package_classifies_as_none() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["empty"]
"#,
    );

    fs::create_dir_all(root.path().join("empty")).expect("create empty dir");
    write(root.path().join("empty/Cargo.toml"), "[bad]\nvalue = 1\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_descendant_root(&input, "empty", "empty/Cargo.toml", None);
    assertions::assert_no_input_failures(&input);
}

#[test]
fn unreferenced_and_excluded_real_child_roots_still_appear() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["declared"]
exclude = ["excluded"]
"#,
    );

    fs::create_dir_all(root.path().join("declared/src")).expect("create declared source dir");
    fs::create_dir_all(root.path().join("excluded/src")).expect("create excluded source dir");
    fs::create_dir_all(root.path().join("stray/src")).expect("create stray source dir");

    write(
        root.path().join("declared/Cargo.toml"),
        r#"
[package]
name = "declared"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("declared/src/lib.rs"),
        "pub struct Declared;\n",
    );

    write(
        root.path().join("excluded/Cargo.toml"),
        r#"
[package]
name = "excluded"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("excluded/src/lib.rs"),
        "pub struct Excluded;\n",
    );

    write(
        root.path().join("stray/Cargo.toml"),
        r#"
[package]
name = "stray"
version = "0.1.0"
"#,
    );
    write(root.path().join("stray/src/lib.rs"), "pub struct Stray;\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    let rel_dirs = input
        .descendant_cargo_roots
        .iter()
        .map(|root| root.rel_dir.as_str())
        .collect::<Vec<_>>();
    assert_eq!(rel_dirs, vec!["declared", "excluded", "stray"]);
}

#[test]
fn ignored_descendant_roots_and_family_files_stay_out() {
    let root = tempdir().expect("create temp dir");
    git_init(root.path());

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["live"]
"#,
    );
    write(root.path().join(".gitignore"), "ignored/\n");

    fs::create_dir_all(root.path().join("live/src")).expect("create live source dir");
    fs::create_dir_all(root.path().join("ignored/src")).expect("create ignored source dir");
    fs::create_dir_all(root.path().join("ignored/.cargo"))
        .expect("create ignored cargo config dir");

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

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

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
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["live"]
"#,
    );

    fs::create_dir_all(root.path().join("live/src")).expect("create live source dir");
    fs::create_dir_all(root.path().join("tests/fixtures/demo/src"))
        .expect("create fixture source dir");
    fs::create_dir_all(root.path().join("tests/snapshots/demo/src"))
        .expect("create snapshot source dir");
    fs::create_dir_all(root.path().join("target/generated/src"))
        .expect("create generated target dir");
    fs::create_dir_all(root.path().join(".claude/worktrees/tmp/src"))
        .expect("create ignored worktree dir");

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

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

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
    assert!(
        input
            .family_files
            .iter()
            .all(|file| !file.rel_path.starts_with("target/"))
    );
    assert!(
        input
            .family_files
            .iter()
            .all(|file| !file.rel_path.starts_with(".claude/worktrees/"))
    );
}

#[test]
fn family_file_mapping_covers_supported_workspace_local_files() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    fs::create_dir_all(root.path().join(".cargo")).expect("create cargo config dir");
    fs::create_dir_all(root.path().join(".config")).expect("create config dir");

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

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "Cargo.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Cargo,
        "Cargo.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Release,
        "Cargo.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Cargo,
        "guardrail3-rs.toml",
        G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deps,
        "guardrail3-rs.toml",
        G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        "guardrail3-rs.toml",
        G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Fmt,
        "rustfmt.toml",
        G3RsTopologyWorkspaceFamilyFileKind::RustfmtToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Fmt,
        ".rustfmt.toml",
        G3RsTopologyWorkspaceFamilyFileKind::DotRustfmtToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "rust-toolchain.toml",
        G3RsTopologyWorkspaceFamilyFileKind::RustToolchainToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "rust-toolchain",
        G3RsTopologyWorkspaceFamilyFileKind::RustToolchainLegacy,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        "clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        ".clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyDotToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        ".cargo/config.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        ".cargo/config.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        ".cargo/config",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigLegacy,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deny,
        "deny.toml",
        G3RsTopologyWorkspaceFamilyFileKind::DenyToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deny,
        ".deny.toml",
        G3RsTopologyWorkspaceFamilyFileKind::DenyDotToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Deny,
        ".cargo/deny.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Release,
        "release-plz.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ReleasePlzToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Release,
        "cliff.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CliffToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Test,
        ".cargo/mutants.toml",
        G3RsTopologyWorkspaceFamilyFileKind::MutantsToml,
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Test,
        ".config/nextest.toml",
        G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
    );

    assertions::assert_exact_family_file_count(&input, "Cargo.toml", 7);
    assertions::assert_exact_family_file_count(&input, "guardrail3.toml", 0);
    assertions::assert_exact_family_file_count(&input, "guardrail3-rs.toml", 3);
    assertions::assert_exact_family_file_count(&input, "clippy.toml", 2);
    assertions::assert_exact_family_file_count(&input, ".cargo/config.toml", 2);
    assertions::assert_exact_family_file_count(&input, ".cargo/config", 2);
    assertions::assert_exact_family_file_count(&input, ".cargo/mutants.toml", 1);
    assertions::assert_exact_family_file_count(&input, ".config/nextest.toml", 1);
}

#[test]
fn descendant_cargo_toml_files_map_to_all_supported_families() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crate_a"]
"#,
    );
    fs::create_dir_all(root.path().join("crate_a/src")).expect("create crate source dir");
    write(
        root.path().join("crate_a/Cargo.toml"),
        r#"
[package]
name = "crate_a"
version = "0.1.0"
"#,
    );
    write(root.path().join("crate_a/src/lib.rs"), "pub struct A;\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    for family in [
        G3RsTopologyWorkspaceFamily::Toolchain,
        G3RsTopologyWorkspaceFamily::Clippy,
        G3RsTopologyWorkspaceFamily::Deny,
        G3RsTopologyWorkspaceFamily::Cargo,
        G3RsTopologyWorkspaceFamily::Deps,
        G3RsTopologyWorkspaceFamily::Garde,
        G3RsTopologyWorkspaceFamily::Release,
    ] {
        assertions::assert_family_file(
            &input,
            family,
            "crate_a/Cargo.toml",
            G3RsTopologyWorkspaceFamilyFileKind::CargoToml,
        );
    }
    assertions::assert_exact_family_file_count(&input, "crate_a/Cargo.toml", 7);
}

#[test]
fn root_read_failure_after_crawl_fails_ingestion() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        "[workspace]\nmembers = []\n",
    );
    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    fs::remove_file(root.path().join("Cargo.toml")).expect("remove root manifest");

    let err = super::super::ingest_for_file_tree_checks(&crawl)
        .expect_err("deleted root manifest should fail ingestion");

    match err {
        g3rs_topology_ingestion_types::G3RsTopologyIngestionError::Unreadable { path, reason } => {
            assert!(path.ends_with("Cargo.toml"));
            assert!(reason.contains("No such file or directory"), "{reason}");
        }
        other => panic!("unexpected ingestion error: {other:?}"),
    }
}

#[test]
fn descendant_read_failure_after_crawl_becomes_input_failure() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["bad"]
"#,
    );
    fs::create_dir_all(root.path().join("bad/src")).expect("create bad source dir");
    write(
        root.path().join("bad/Cargo.toml"),
        r#"
[package]
name = "bad"
version = "0.1.0"
"#,
    );
    write(root.path().join("bad/src/lib.rs"), "pub struct Bad;\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    fs::remove_file(root.path().join("bad/Cargo.toml")).expect("remove bad manifest");

    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_input_failure_contains(
        &input,
        "bad/Cargo.toml",
        "No such file or directory",
    );
    assertions::assert_descendant_root(&input, "bad", "bad/Cargo.toml", None);
}

#[test]
fn file_tree_input_preserves_workspace_member_exactness_shapes() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/*", "missing", "shared"]
"#,
    );

    fs::create_dir_all(root.path().join("crates/app/src")).expect("create app source dir");
    fs::create_dir_all(root.path().join("crates/extra/src")).expect("create extra source dir");
    fs::create_dir_all(root.path().join("shared/src")).expect("create shared source dir");

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
        root.path().join("crates/extra/Cargo.toml"),
        r#"
[package]
name = "extra"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("crates/extra/src/lib.rs"),
        "pub struct Extra;\n",
    );

    write(
        root.path().join("shared/Cargo.toml"),
        r#"
[package]
name = "shared"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("shared/src/lib.rs"),
        "pub struct Shared;\n",
    );

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");
    let workspace = input
        .workspace_manifest
        .workspace
        .as_ref()
        .expect("workspace manifest should parse as workspace");

    assert_eq!(
        workspace.members,
        vec![
            "crates/*".to_owned(),
            "missing".to_owned(),
            "shared".to_owned()
        ]
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .any(|root| root.rel_dir == "crates/app")
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .any(|root| root.rel_dir == "crates/extra")
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .any(|root| root.rel_dir == "shared")
    );
}

#[test]
fn file_tree_input_preserves_escaping_member_patterns() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["crates/*", "../shared"]
"#,
    );

    fs::create_dir_all(root.path().join("crates/app/src")).expect("create app source dir");
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

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");
    let workspace = input
        .workspace_manifest
        .workspace
        .as_ref()
        .expect("workspace manifest should parse as workspace");

    assert_eq!(
        workspace.members,
        vec!["crates/*".to_owned(), "../shared".to_owned()]
    );
    assert!(
        input
            .descendant_cargo_roots
            .iter()
            .all(|root| root.rel_dir != "../shared")
    );
}

#[test]
fn file_tree_input_preserves_illegal_family_file_placement_shapes() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["apps/api", "packages/lib"]
"#,
    );

    fs::create_dir_all(root.path().join("apps/api/src")).expect("create api source dir");
    fs::create_dir_all(root.path().join("packages/lib/src")).expect("create lib source dir");
    fs::create_dir_all(root.path().join("packages/lib/.cargo"))
        .expect("create lib cargo config dir");
    fs::create_dir_all(root.path().join("tools/helper")).expect("create helper dir");
    fs::create_dir_all(root.path().join("apps/api/nested")).expect("create nested dir");

    write(
        root.path().join("apps/api/Cargo.toml"),
        r#"
[package]
name = "api"
version = "0.1.0"
"#,
    );
    write(root.path().join("apps/api/src/lib.rs"), "pub struct Api;\n");

    write(
        root.path().join("packages/lib/Cargo.toml"),
        r#"
[package]
name = "lib"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("packages/lib/src/lib.rs"),
        "pub struct Lib;\n",
    );

    write(
        root.path().join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"stable\"\n",
    );
    write(
        root.path().join("apps/api/nested/clippy.toml"),
        "msrv = \"1.85\"\n",
    );
    write(
        root.path().join("packages/lib/.cargo/config.toml"),
        "[alias]\n",
    );
    write(
        root.path().join("tools/helper/guardrail3-rs.toml"),
        "profile = \"service\"\n",
    );

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "rust-toolchain.toml",
        G3RsTopologyWorkspaceFamilyFileKind::RustToolchainToml,
    );
    assertions::assert_family_file_attachment(
        &input,
        G3RsTopologyWorkspaceFamily::Toolchain,
        "rust-toolchain.toml",
        G3RsTopologyWorkspaceFamilyFileKind::RustToolchainToml,
        G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
            root_rel: String::new(),
        },
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "apps/api/nested/clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
    );
    assertions::assert_family_file_attachment(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "apps/api/nested/clippy.toml",
        G3RsTopologyWorkspaceFamilyFileKind::ClippyToml,
        G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
            root_rel: "apps/api".to_owned(),
            owner_rel: "apps/api/nested".to_owned(),
        },
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "packages/lib/.cargo/config.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
    );
    assertions::assert_family_file_attachment(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "packages/lib/.cargo/config.toml",
        G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
        G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
            root_rel: "packages/lib".to_owned(),
        },
    );
    assertions::assert_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        "tools/helper/guardrail3-rs.toml",
        G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
    );
    assertions::assert_family_file_attachment(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        "tools/helper/guardrail3-rs.toml",
        G3RsTopologyWorkspaceFamilyFileKind::Guardrail3RsToml,
        G3RsTopologyWorkspaceFamilyFileAttachment::NestedUnderRoot {
            root_rel: String::new(),
            owner_rel: "tools/helper".to_owned(),
        },
    );
}

#[test]
fn attachment_normalizes_cargo_and_config_owner_dirs() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["member"]
"#,
    );
    fs::create_dir_all(root.path().join("member/src")).expect("create member source dir");
    fs::create_dir_all(root.path().join("member/.cargo")).expect("create member cargo config dir");
    fs::create_dir_all(root.path().join("member/.config")).expect("create member config dir");

    write(
        root.path().join("member/Cargo.toml"),
        r#"
[package]
name = "member"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("member/src/lib.rs"),
        "pub struct Member;\n",
    );
    write(root.path().join("member/.cargo/config.toml"), "[alias]\n");
    write(
        root.path().join("member/.cargo/deny.toml"),
        "advisories = {}\n",
    );
    write(
        root.path().join("member/.cargo/mutants.toml"),
        "timeout_multiplier = 2.0\n",
    );
    write(
        root.path().join("member/.config/nextest.toml"),
        "[profile.default]\n",
    );

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    for (family, rel_path, kind) in [
        (
            G3RsTopologyWorkspaceFamily::Clippy,
            "member/.cargo/config.toml",
            G3RsTopologyWorkspaceFamilyFileKind::CargoConfigToml,
        ),
        (
            G3RsTopologyWorkspaceFamily::Deny,
            "member/.cargo/deny.toml",
            G3RsTopologyWorkspaceFamilyFileKind::CargoDenyToml,
        ),
        (
            G3RsTopologyWorkspaceFamily::Test,
            "member/.cargo/mutants.toml",
            G3RsTopologyWorkspaceFamilyFileKind::MutantsToml,
        ),
        (
            G3RsTopologyWorkspaceFamily::Test,
            "member/.config/nextest.toml",
            G3RsTopologyWorkspaceFamilyFileKind::NextestToml,
        ),
    ] {
        assertions::assert_family_file_attachment(
            &input,
            family,
            rel_path,
            kind,
            G3RsTopologyWorkspaceFamilyFileAttachment::ExactRoot {
                root_rel: "member".to_owned(),
            },
        );
    }
}

#[test]
fn file_tree_ingestion_precomputes_atomic_check_inputs() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["apps/api", "apps/nested", "../shared"]
"#,
    );
    fs::create_dir_all(root.path().join("apps/api/src")).expect("create api source dir");
    fs::create_dir_all(root.path().join("apps/nested/src")).expect("create nested source dir");
    fs::create_dir_all(root.path().join("packages/lib/src")).expect("create lib source dir");
    fs::create_dir_all(root.path().join("packages/lib/.cargo"))
        .expect("create lib cargo config dir");
    fs::create_dir_all(root.path().join("tools/helper")).expect("create helper dir");

    write(
        root.path().join("apps/api/Cargo.toml"),
        r#"
[package]
name = "api"
version = "0.1.0"
"#,
    );
    write(root.path().join("apps/api/src/lib.rs"), "pub struct Api;\n");

    write(
        root.path().join("apps/nested/Cargo.toml"),
        r"
[workspace]
members = []
",
    );
    write(
        root.path().join("apps/nested/src/lib.rs"),
        "pub struct Nested;\n",
    );

    write(
        root.path().join("packages/lib/Cargo.toml"),
        r#"
[package]
name = "lib"
version = "0.1.0"
"#,
    );
    write(
        root.path().join("packages/lib/src/lib.rs"),
        "pub struct Lib;\n",
    );

    write(
        root.path().join("packages/lib/.cargo/config.toml"),
        "[alias]\n",
    );
    write(
        root.path().join("tools/helper/guardrail3-rs.toml"),
        "profile = \"service\"\n",
    );

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_nested_workspace(&input, "apps/nested", "apps/nested/Cargo.toml", "");
    assertions::assert_undeclared_member_issue(
        &input,
        "packages/lib",
        "packages/lib/Cargo.toml",
        "",
    );
    assertions::assert_extra_member_issue(&input, "Cargo.toml", "", "apps/nested");
    assertions::assert_escaping_member_path(&input, "Cargo.toml", "", "../shared");
    assertions::assert_illegal_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Clippy,
        "packages/lib/.cargo/config.toml",
        "attached to illegal child root `packages/lib`",
    );
    assertions::assert_illegal_family_file(
        &input,
        G3RsTopologyWorkspaceFamily::Garde,
        "tools/helper/guardrail3-rs.toml",
        "nested under `tools/helper`",
    );
}

#[test]
fn nested_guardrail3_rs_toml_under_adopted_outer_fires() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["inner"]
"#,
    );
    write(root.path().join("guardrail3-rs.toml"), "[rust]\n");

    fs::create_dir_all(root.path().join("inner/src")).expect("create inner source dir");
    write(
        root.path().join("inner/Cargo.toml"),
        r"
[workspace]
members = []
",
    );
    write(root.path().join("inner/src/lib.rs"), "pub struct Inner;\n");
    write(root.path().join("inner/guardrail3-rs.toml"), "[rust]\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_nested_guardrail3_rs_toml(&input, "inner", "inner/guardrail3-rs.toml", "");
}

#[test]
fn sibling_guardrail3_rs_tomls_do_not_fire() {
    // reason: ingestion runs rooted at one adopted unit. Two adopted siblings under a
    // non-adopted parent are not visible to each other's run; each run sees no descendant
    // guardrail3-rs.toml. The non-adopted parent run is covered by the
    // `non_adopted_outer_does_not_fire_for_inner_guardrail3_rs_toml` test.
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r"
[workspace]
members = []
",
    );
    write(root.path().join("guardrail3-rs.toml"), "[rust]\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_no_nested_guardrail3_rs_tomls(&input);
}

#[test]
fn non_adopted_outer_does_not_fire_for_inner_guardrail3_rs_toml() {
    let root = tempdir().expect("create temp dir");

    write(
        root.path().join("Cargo.toml"),
        r#"
[workspace]
members = ["inner"]
"#,
    );
    // reason: outer has no sibling guardrail3-rs.toml; outer is not adopted.

    fs::create_dir_all(root.path().join("inner/src")).expect("create inner source dir");
    write(
        root.path().join("inner/Cargo.toml"),
        r"
[workspace]
members = []
",
    );
    write(root.path().join("inner/src/lib.rs"), "pub struct Inner;\n");
    write(root.path().join("inner/guardrail3-rs.toml"), "[rust]\n");

    let crawl = crawl(root.path()).expect("crawl workspace fixture before ingestion");
    let input =
        super::super::ingest_for_file_tree_checks(&crawl).expect("ingest topology file-tree facts");

    assertions::assert_no_nested_guardrail3_rs_tomls(&input);
}

fn write(path: std::path::PathBuf, content: &str) {
    fs::write(path, content).expect("write fixture file");
}

#[cfg(unix)]
fn make_unreadable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)
        .expect("read file metadata before making file unreadable")
        .permissions();
    perms.set_mode(0o000);
    fs::set_permissions(path, perms).expect("set file unreadable");
}

#[cfg(not(unix))]
fn make_unreadable(_path: &std::path::Path) {
    panic!("unreadable test requires unix permissions");
}

#[cfg(unix)]
fn restore_readable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)
        .expect("read file metadata before restoring readability")
        .permissions();
    perms.set_mode(0o644);
    fs::set_permissions(path, perms).expect("restore file readability");
}

#[cfg(not(unix))]
fn restore_readable(_path: &std::path::Path) {}

fn git_init(root: &std::path::Path) {
    let status = std::process::Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(root)
        .status()
        .expect("run `git init` for topology ingestion fixture");
    assert!(status.success(), "git init should succeed");
}
