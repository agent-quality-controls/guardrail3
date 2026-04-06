use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

// ---------------------------------------------------------------------------
// Happy path: basic ingestion
// ---------------------------------------------------------------------------

#[test]
fn ingests_toolchain_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed for a valid rust-toolchain.toml");

    assert_eq!(
        output.toolchain_rel_path, "rust-toolchain.toml",
        "toolchain_rel_path should be the workspace-root-relative path"
    );
    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("parsed toolchain_toml should contain a [toolchain] section");
    assert_eq!(
        section.channel.as_deref(),
        Some("1.85.0"),
        "parsed channel should be 1.85.0"
    );
    assert!(
        output.cargo_toml.is_none(),
        "cargo_toml should be None when Cargo.toml is absent"
    );
    assert!(
        output.cargo_rel_path.is_none(),
        "cargo_rel_path should be None when Cargo.toml is absent"
    );
}

// ---------------------------------------------------------------------------
// Happy path: both files present
// ---------------------------------------------------------------------------

#[test]
fn ingests_with_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );
    write(
        root.join("Cargo.toml"),
        "[workspace]\nresolver = \"2\"\n\n[workspace.package]\nrust-version = \"1.85\"\n",
    );

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed when both config files are present");

    assert_eq!(output.toolchain_rel_path, "rust-toolchain.toml");
    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("should have a [toolchain] section");
    assert_eq!(section.channel.as_deref(), Some("1.85.0"));

    assert_eq!(
        output.cargo_rel_path.as_deref(),
        Some("Cargo.toml"),
        "cargo_rel_path should be set when Cargo.toml is present"
    );
    let cargo = output
        .cargo_toml
        .as_ref()
        .expect("cargo_toml should be Some when Cargo.toml is present and parseable");
    let rust_version = cargo
        .workspace
        .as_ref()
        .and_then(|ws| ws.package.as_ref())
        .and_then(|pkg| pkg.rust_version.as_deref());
    assert_eq!(
        rust_version,
        Some("1.85"),
        "should carry the parsed rust-version from Cargo.toml"
    );
}

// ---------------------------------------------------------------------------
// Cargo.toml absent → cargo fields are None
// ---------------------------------------------------------------------------

#[test]
fn cargo_none_without_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed even without Cargo.toml");

    assert!(output.cargo_toml.is_none());
    assert!(output.cargo_rel_path.is_none());
    assert_eq!(output.toolchain_rel_path, "rust-toolchain.toml");
    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("should have a [toolchain] section");
    assert_eq!(section.channel.as_deref(), Some("1.85.0"));
}

// ---------------------------------------------------------------------------
// Error: toolchain file missing
// ---------------------------------------------------------------------------

#[test]
fn fails_when_toolchain_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let crawl = crawl(root);
    let result = crate::ingest(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::ToolchainTomlNotFound)),
        "ingestion should return ToolchainTomlNotFound when no rust-toolchain.toml exists"
    );
}

// ---------------------------------------------------------------------------
// Error: malformed toolchain file
// ---------------------------------------------------------------------------

#[test]
fn fails_on_malformed_toolchain_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "{{{{not valid toml at all}}}}",
    );

    let crawl = crawl(root);
    let err = crate::ingest(&crawl)
        .expect_err("ingestion should return Err when rust-toolchain.toml contains invalid TOML");

    match &err {
        crate::IngestionError::ParseFailed { path, reason } => {
            assert!(
                path.ends_with("rust-toolchain.toml"),
                "ParseFailed path should reference rust-toolchain.toml, got: {path:?}"
            );
            assert!(!reason.is_empty(), "ParseFailed reason should not be empty");
        }
        other => panic!("expected ParseFailed, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Recovery: gitignored toolchain file recovered by crawl
// ---------------------------------------------------------------------------

#[test]
fn ignored_but_recovered_toolchain_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "rust-toolchain.toml\n");
    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );

    let crawl = crawl(root);
    let output = crate::ingest(&crawl).expect(
        "ingestion should succeed for a gitignored rust-toolchain.toml recovered by crawl",
    );

    assert_eq!(output.toolchain_rel_path, "rust-toolchain.toml");
    assert!(output.cargo_toml.is_none());
    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("recovered toolchain should parse correctly");
    assert_eq!(section.channel.as_deref(), Some("1.85.0"));
}

// ---------------------------------------------------------------------------
// Graceful degradation: malformed Cargo.toml → cargo fields None
// ---------------------------------------------------------------------------

#[test]
fn malformed_cargo_toml_produces_cargo_none() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );
    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let output = crate::ingest(&crawl).expect(
        "ingestion should succeed even when Cargo.toml is malformed — graceful degradation",
    );

    assert!(output.cargo_toml.is_none());
    assert!(output.cargo_rel_path.is_none());
    assert_eq!(output.toolchain_rel_path, "rust-toolchain.toml");
}

// ---------------------------------------------------------------------------
// Toolchain with components but no channel
// ---------------------------------------------------------------------------

#[test]
fn toolchain_with_components_only() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\ncomponents = [\"rustfmt\", \"clippy\"]\n",
    );

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed for a toolchain with components but no channel");

    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("should have a [toolchain] section");
    assert!(section.channel.is_none());
    assert_eq!(
        section.components,
        vec!["rustfmt".to_owned(), "clippy".to_owned()],
    );
}

// ---------------------------------------------------------------------------
// Empty toolchain file (valid TOML, no [toolchain] section)
// ---------------------------------------------------------------------------

#[test]
fn empty_toolchain_file_succeeds() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rust-toolchain.toml"), "");

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed for an empty rust-toolchain.toml");

    assert!(output.toolchain_toml.toolchain.is_none());
}

// ---------------------------------------------------------------------------
// Toolchain with extra unknown keys (forward compatibility)
// ---------------------------------------------------------------------------

#[test]
fn toolchain_with_extra_keys_succeeds() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"nightly\"\n\n[some-future-section]\nfoo = \"bar\"\n",
    );

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed for a toolchain file with extra sections");

    let section = output
        .toolchain_toml
        .toolchain
        .as_ref()
        .expect("should still parse the [toolchain] section");
    assert_eq!(section.channel.as_deref(), Some("nightly"));
    assert!(
        !output.toolchain_toml.extra.is_empty(),
        "extra keys should be preserved"
    );
}

// ---------------------------------------------------------------------------
// Error Display impls produce meaningful messages
// ---------------------------------------------------------------------------

#[test]
fn error_display_toolchain_not_found() {
    let err = crate::IngestionError::ToolchainTomlNotFound;
    let msg = err.to_string();
    assert!(
        msg.contains("rust-toolchain.toml"),
        "ToolchainTomlNotFound display should mention rust-toolchain.toml, got: {msg}"
    );
}

#[test]
fn error_display_unreadable() {
    let err = crate::IngestionError::Unreadable {
        path: std::path::PathBuf::from("/fake/rust-toolchain.toml"),
        reason: "permission denied".to_owned(),
    };
    let msg = err.to_string();
    assert!(msg.contains("rust-toolchain.toml"), "got: {msg}");
    assert!(msg.contains("permission denied"), "got: {msg}");
}

#[test]
fn error_display_parse_failed() {
    let err = crate::IngestionError::ParseFailed {
        path: std::path::PathBuf::from("/fake/rust-toolchain.toml"),
        reason: "expected `=`".to_owned(),
    };
    let msg = err.to_string();
    assert!(msg.contains("rust-toolchain.toml"), "got: {msg}");
    assert!(msg.contains("expected `=`"), "got: {msg}");
}

// ---------------------------------------------------------------------------
// Error type implements std::error::Error
// ---------------------------------------------------------------------------

#[test]
fn error_implements_std_error() {
    fn assert_error<T: std::error::Error>() {}
    assert_error::<crate::IngestionError>();
}

// ---------------------------------------------------------------------------
// Unreadable toolchain file (readable flag = false) → Unreadable error
// ---------------------------------------------------------------------------

#[test]
fn unreadable_toolchain_toml_returns_error() {
    use g3rs_workspace_crawl::{
        G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind,
        G3RsWorkspaceIgnoreState, G3RsWorkspacePath,
    };

    let crawl = G3RsWorkspaceCrawl {
        root_abs_path: std::path::PathBuf::from("/synthetic/workspace"),
        entries: vec![G3RsWorkspaceEntry {
            path: G3RsWorkspacePath {
                rel_path: "rust-toolchain.toml".to_owned(),
                abs_path: std::path::PathBuf::from("/synthetic/workspace/rust-toolchain.toml"),
            },
            kind: G3RsWorkspaceEntryKind::File,
            ignore_state: G3RsWorkspaceIgnoreState::Included,
            readable: false,
        }],
    };

    let err = crate::ingest(&crawl)
        .expect_err("ingestion should return Err when rust-toolchain.toml has readable=false");

    match &err {
        crate::IngestionError::Unreadable { path, reason } => {
            assert!(path.ends_with("rust-toolchain.toml"), "got: {path:?}");
            assert!(!reason.is_empty());
        }
        other => panic!("expected Unreadable, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// Unreadable Cargo.toml (readable flag = false) → cargo fields None
// ---------------------------------------------------------------------------

#[test]
fn unreadable_cargo_toml_produces_cargo_none() {
    use g3rs_workspace_crawl::{
        G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind,
        G3RsWorkspaceIgnoreState, G3RsWorkspacePath,
    };

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );

    let crawl = G3RsWorkspaceCrawl {
        root_abs_path: root.to_path_buf(),
        entries: vec![
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: "rust-toolchain.toml".to_owned(),
                    abs_path: root.join("rust-toolchain.toml"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: true,
            },
            G3RsWorkspaceEntry {
                path: G3RsWorkspacePath {
                    rel_path: "Cargo.toml".to_owned(),
                    abs_path: root.join("Cargo.toml"),
                },
                kind: G3RsWorkspaceEntryKind::File,
                ignore_state: G3RsWorkspaceIgnoreState::Included,
                readable: false,
            },
        ],
    };

    let output = crate::ingest(&crawl).expect(
        "ingestion should succeed when Cargo.toml is unreadable — graceful degradation",
    );

    assert!(output.cargo_toml.is_none());
    assert!(output.cargo_rel_path.is_none());
    assert_eq!(output.toolchain_rel_path, "rust-toolchain.toml");
}

// ---------------------------------------------------------------------------
// TOCTOU: toolchain file deleted between crawl and read → Unreadable error
// ---------------------------------------------------------------------------

#[test]
fn toolchain_deleted_after_crawl_returns_unreadable() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );

    let crawl = crawl(root);
    fs::remove_file(root.join("rust-toolchain.toml"))
        .expect("should delete rust-toolchain.toml for TOCTOU test");

    let err = crate::ingest(&crawl).expect_err(
        "ingestion should return Err when rust-toolchain.toml is deleted between crawl and read",
    );

    match &err {
        crate::IngestionError::Unreadable { path, reason } => {
            assert!(path.ends_with("rust-toolchain.toml"), "got: {path:?}");
            assert!(!reason.is_empty(), "got: {reason}");
        }
        other => panic!("expected Unreadable from TOCTOU, got: {other:?}"),
    }
}

// ---------------------------------------------------------------------------
// TOCTOU: Cargo.toml deleted between crawl and read → cargo fields None
// ---------------------------------------------------------------------------

#[test]
fn cargo_deleted_after_crawl_produces_cargo_none() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );
    write(root.join("Cargo.toml"), "[workspace]\nresolver = \"2\"\n");

    let crawl = crawl(root);
    fs::remove_file(root.join("Cargo.toml"))
        .expect("should delete Cargo.toml for TOCTOU test");

    let output = crate::ingest(&crawl).expect(
        "ingestion should succeed when Cargo.toml vanishes after crawl — graceful degradation",
    );

    assert!(output.cargo_toml.is_none());
    assert!(output.cargo_rel_path.is_none());
}

// ---------------------------------------------------------------------------
// Cargo.toml present but no rust-version field
// ---------------------------------------------------------------------------

#[test]
fn cargo_toml_without_rust_version() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );
    write(root.join("Cargo.toml"), "[workspace]\nresolver = \"2\"\n");

    let crawl = crawl(root);
    let output = crate::ingest(&crawl)
        .expect("ingestion should succeed when Cargo.toml has no rust-version");

    let cargo = output
        .cargo_toml
        .as_ref()
        .expect("cargo_toml should be Some — Cargo.toml exists and parses");
    let rust_version = cargo
        .workspace
        .as_ref()
        .and_then(|ws| ws.package.as_ref())
        .and_then(|pkg| pkg.rust_version.as_deref());
    assert_eq!(rust_version, None);
}
