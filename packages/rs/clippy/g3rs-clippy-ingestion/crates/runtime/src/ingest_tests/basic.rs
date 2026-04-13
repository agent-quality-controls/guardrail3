use std::fs;
use std::path::Path;
use std::process::Command;

use g3rs_clippy_types::G3RsClippyConfigState;
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

fn typed_msrv(input: &g3rs_clippy_types::G3RsClippyConfigChecksInput) -> Option<&str> {
    match &input.clippy {
        G3RsClippyConfigState::Parsed {
            typed: Ok(clippy), ..
        } => clippy.msrv.as_deref(),
        G3RsClippyConfigState::Unreadable { .. }
        | G3RsClippyConfigState::ParseError { .. }
        | G3RsClippyConfigState::Parsed { typed: Err(_), .. } => None,
    }
}

#[test]
fn ingests_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for a valid clippy.toml");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy_rel_path should be the workspace-root-relative path"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.85"),
        "parsed ClippyToml should contain the msrv value from the fixture file"
    );
}

#[test]
fn ingests_dot_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for a valid .clippy.toml");
    assert_eq!(
        input.clippy_rel_path, ".clippy.toml",
        "clippy_rel_path should be the dot-prefixed variant when only .clippy.toml exists"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.85"),
        "parsed ClippyToml should contain the msrv value from the dot-prefixed fixture file"
    );
}

#[test]
fn prefers_dot_clippy_toml_over_plain_variant() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.80\"\n");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed when both clippy config variants exist");
    assert_eq!(
        input.clippy_rel_path, ".clippy.toml",
        ".clippy.toml should win same-root precedence when both variants exist"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.80"),
        "parsed content should come from .clippy.toml (1.80), not clippy.toml (1.85)"
    );
}

#[test]
fn keeps_raw_parseable_but_typed_invalid_clippy_for_config_checks() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "disallowed-methods = [{ path = 7 }]\n",
    );

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect(
        "config ingestion should preserve raw-parseable clippy.toml for parseability and section-shape checks instead of aborting on typed parse failure",
    );
    match input.clippy {
        G3RsClippyConfigState::Parsed {
            typed: Err(reason),
            ..
        } => assert!(
            reason.contains("path"),
            "typed parse error should preserve the parser reason: {reason}"
        ),
        other => panic!("expected raw-parseable typed-invalid clippy state, got {other:#?}"),
    }
}

#[test]
fn filetree_ingestion_is_not_a_stub_when_root_configs_exist() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nmembers = []\n");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");
    write(root.join(".clippy.toml"), "msrv = \"1.80\"\n");

    let crawl = crawl(root);
    let input = crate::ingest_for_file_tree_checks(&crawl).expect(
        "filetree ingestion should build real root coverage and same-root conflict facts instead of returning a stub error",
    );
    assert_eq!(
        input,
        g3rs_clippy_types::G3RsClippyFileTreeChecksInput {
            preferred_root_config_rel_path: Some(".clippy.toml".to_owned()),
            shadowed_same_root_configs: vec![g3rs_clippy_types::G3RsClippyShadowedConfig {
                rel_path: "clippy.toml".to_owned(),
                preferred_rel_path: ".clippy.toml".to_owned(),
            }],
        }
    );
}

#[test]
fn fails_when_clippy_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ClippyTomlNotFound)
        ),
        "ingestion should return ClippyTomlNotFound when no clippy config exists in the workspace"
    );
}

#[test]
fn keeps_raw_parse_error_state_for_malformed_clippy_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "{{{{not valid toml at all}}}}");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve malformed clippy.toml state");
    assert!(
        matches!(input.clippy, G3RsClippyConfigState::ParseError { .. }),
        "{input:#?}"
    );
}

#[test]
fn keeps_typed_parse_error_state_for_unknown_fields() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("clippy.toml"),
        "totally_fake_field = true\n",
    );

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve typed parse errors for unknown fields");
    assert!(
        matches!(
            input.clippy,
            G3RsClippyConfigState::Parsed {
                typed: Err(_),
                ..
            }
        ),
        "{input:#?}"
    );
}

#[test]
fn keeps_typed_parse_error_state_for_wrong_value_type() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "msrv = 42\n");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should preserve typed parse errors for wrong value types");
    assert!(
        matches!(
            input.clippy,
            G3RsClippyConfigState::Parsed {
                typed: Err(_),
                ..
            }
        ),
        "{input:#?}"
    );
}

#[test]
fn empty_clippy_toml_parses_to_defaults() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("clippy.toml"), "");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect("ingestion should succeed for an empty clippy.toml");
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "clippy_rel_path should be the workspace-root-relative path"
    );
    assert!(
        typed_msrv(&input).is_none(),
        "empty clippy.toml should have no msrv set"
    );
}

#[test]
fn nested_clippy_toml_is_not_selected() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(
        root.join("subdir/clippy.toml"),
        "msrv = \"1.85\"\n",
    );

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(
            result,
            Err(crate::IngestionError::ClippyTomlNotFound)
        ),
        "ingestion should not select a clippy.toml in a subdirectory, only at the workspace root"
    );
}

#[test]
fn malformed_root_cargo_toml_does_not_abort_clippy_config_ingestion() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), "[workspace]\nnot = [valid");
    write(root.join("guardrail3.toml"), "[profile]\nname = \"library\"\n");
    write(root.join("clippy.toml"), "avoid-breaking-exported-api = true\n");

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("malformed root Cargo.toml should disable published-library policy, not abort clippy ingestion");

    assert!(!input.published_library_policy, "{input:#?}");
}

#[test]
fn ignored_but_recovered_clippy_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "clippy.toml\n");
    write(root.join("clippy.toml"), "msrv = \"1.85\"\n");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let input = result.expect(
        "ingestion should succeed for a gitignored clippy.toml recovered by the crawl recovery phase",
    );
    assert_eq!(
        input.clippy_rel_path, "clippy.toml",
        "recovered clippy.toml should still resolve to the root-relative path"
    );
    assert_eq!(
        typed_msrv(&input),
        Some("1.85"),
        "recovered clippy.toml should be parsed correctly"
    );
}
