#![allow(
    clippy::disallowed_methods,
    reason = "fixture-driven filesystem tests need direct std::fs calls in test bodies"
)]

use std::fs;

use tempfile::tempdir;

use super::helpers::{crawl, git_init, write};

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
    let output = super::ingest_for_config_checks(&crawl)
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
    let output = super::ingest_for_config_checks(&crawl)
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
    let output = super::ingest_for_config_checks(&crawl)
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
    let result = super::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::ToolchainTomlNotFound)),
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
    let err = super::ingest_for_config_checks(&crawl)
        .expect_err("ingestion should return Err when rust-toolchain.toml contains invalid TOML");

    assert!(
        matches!(&err, super::IngestionError::ParseFailed { .. }),
        "expected ParseFailed variant from ingestion, got: {err:?}",
    );
    if let super::IngestionError::ParseFailed { path, reason } = &err {
        assert!(
            path.ends_with("rust-toolchain.toml"),
            "ParseFailed path should reference rust-toolchain.toml, got: {path:?}",
        );
        assert!(!reason.is_empty(), "ParseFailed reason should not be empty");
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
    let output = super::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed for a gitignored rust-toolchain.toml recovered by crawl");

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
// Present malformed Cargo.toml fails closed
// ---------------------------------------------------------------------------

#[test]
fn malformed_cargo_toml_returns_error() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );
    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let result = super::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(super::IngestionError::ParseFailed { .. })),
        "ingestion should fail closed when Cargo.toml exists but is malformed"
    );
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
    let output = super::ingest_for_config_checks(&crawl)
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
    let output = super::ingest_for_config_checks(&crawl)
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
    let output = super::ingest_for_config_checks(&crawl)
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
    let err = super::IngestionError::ToolchainTomlNotFound;
    let msg = err.to_string();
    assert!(
        msg.contains("rust-toolchain.toml"),
        "ToolchainTomlNotFound display should mention rust-toolchain.toml, got: {msg}"
    );
}

#[test]
fn error_display_unreadable() {
    let err = super::IngestionError::Unreadable {
        path: std::path::PathBuf::from("/fake/rust-toolchain.toml"),
        reason: "permission denied".to_owned(),
    };
    let msg = err.to_string();
    assert!(msg.contains("rust-toolchain.toml"), "got: {msg}");
    assert!(msg.contains("permission denied"), "got: {msg}");
}

#[test]
fn error_display_parse_failed() {
    let err = super::IngestionError::ParseFailed {
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
    let error: &dyn std::error::Error = &super::IngestionError::ToolchainTomlNotFound;
    assert!(error.to_string().contains("rust-toolchain.toml"), "{error}");
}

// ---------------------------------------------------------------------------
// Unreadable toolchain file (readable flag = false) → Unreadable error
// ---------------------------------------------------------------------------

#[test]
fn unreadable_toolchain_toml_returns_error() {
    use g3rs_workspace_crawl::{
        G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
        G3RsWorkspacePath,
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

    let err = super::ingest_for_config_checks(&crawl)
        .expect_err("ingestion should return Err when rust-toolchain.toml has readable=false");

    assert!(
        matches!(&err, super::IngestionError::Unreadable { .. }),
        "expected Unreadable variant, got: {err:?}",
    );
    if let super::IngestionError::Unreadable { path, reason } = &err {
        assert!(path.ends_with("rust-toolchain.toml"), "got: {path:?}");
        assert!(!reason.is_empty(), "Unreadable reason should not be empty");
    }
}

// ---------------------------------------------------------------------------
// Unreadable Cargo.toml (readable flag = false) → Unreadable error
// ---------------------------------------------------------------------------

#[test]
fn unreadable_cargo_toml_returns_error() {
    use g3rs_workspace_crawl::{
        G3RsWorkspaceCrawl, G3RsWorkspaceEntry, G3RsWorkspaceEntryKind, G3RsWorkspaceIgnoreState,
        G3RsWorkspacePath,
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

    let err = super::ingest_for_config_checks(&crawl)
        .expect_err("ingestion should fail closed when Cargo.toml is unreadable");

    assert!(
        matches!(&err, super::IngestionError::Unreadable { .. }),
        "expected Unreadable variant, got: {err:?}",
    );
    if let super::IngestionError::Unreadable { path, reason } = &err {
        assert!(path.ends_with("Cargo.toml"), "got: {path:?}");
        assert!(!reason.is_empty(), "got: {reason}");
    }
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

    let err = super::ingest_for_config_checks(&crawl).expect_err(
        "ingestion should return Err when rust-toolchain.toml is deleted between crawl and read",
    );

    assert!(
        matches!(&err, super::IngestionError::Unreadable { .. }),
        "expected Unreadable variant from TOCTOU, got: {err:?}",
    );
    if let super::IngestionError::Unreadable { path, reason } = &err {
        assert!(path.ends_with("rust-toolchain.toml"), "got: {path:?}");
        assert!(!reason.is_empty(), "got: {reason}");
    }
}

// ---------------------------------------------------------------------------
// TOCTOU: Cargo.toml deleted between crawl and read → Unreadable error
// ---------------------------------------------------------------------------

#[test]
fn cargo_deleted_after_crawl_returns_unreadable() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("rust-toolchain.toml"),
        "[toolchain]\nchannel = \"1.85.0\"\n",
    );
    write(root.join("Cargo.toml"), "[workspace]\nresolver = \"2\"\n");

    let crawl = crawl(root);
    fs::remove_file(root.join("Cargo.toml")).expect("should delete Cargo.toml for TOCTOU test");

    let err = super::ingest_for_config_checks(&crawl)
        .expect_err("ingestion should fail closed when Cargo.toml vanishes after crawl");

    assert!(
        matches!(&err, super::IngestionError::Unreadable { .. }),
        "expected Unreadable variant, got: {err:?}",
    );
    if let super::IngestionError::Unreadable { path, reason } = &err {
        assert!(path.ends_with("Cargo.toml"), "got: {path:?}");
        assert!(!reason.is_empty(), "got: {reason}");
    }
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
    let output = super::ingest_for_config_checks(&crawl)
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
