use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

/// Minimal valid `rustfmt.toml` content (enough to parse, one field).
const RUSTFMT_CONTENT: &str = "edition = \"2024\"\n";

/// Minimal valid `Cargo.toml` content (workspace with no members).
const CARGO_CONTENT: &str = "[workspace]\nmembers = []\nresolver = \"2\"\n";

/// Minimal valid `rust-toolchain.toml` content.
const TOOLCHAIN_CONTENT: &str = "[toolchain]\nchannel = \"1.85.0\"\n";

/// Realistic `rustfmt.toml` matching the actual repo config with all 8 policy
/// settings that the checks package reads via RS-FMT-CONFIG-01.
const REALISTIC_RUSTFMT: &str = "\
edition = \"2024\"\n\
style_edition = \"2024\"\n\
max_width = 100\n\
tab_spaces = 4\n\
use_field_init_shorthand = true\n\
use_try_shorthand = true\n\
reorder_imports = true\n\
reorder_modules = true\n";

/// Realistic `Cargo.toml` with `[workspace.package]` section that the checks
/// package reads for edition mismatch detection (RS-FMT-CONFIG-04).
const REALISTIC_CARGO: &str = "\
[workspace]\n\
members = []\n\
resolver = \"2\"\n\
\n\
[workspace.package]\n\
version = \"0.1.0\"\n\
edition = \"2024\"\n\
publish = false\n";

/// Realistic `rust-toolchain.toml` matching the actual repo pattern with
/// components, which the checks package reads for nightly detection
/// (RS-FMT-CONFIG-03).
const REALISTIC_TOOLCHAIN: &str = "\
[toolchain]\n\
channel = \"stable\"\n\
components = [\"clippy\", \"rustfmt\"]\n";

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

/// Write all three required config files to the workspace root.
fn write_all_configs(root: &Path) {
    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);
}

/// Assert all three rel_paths in a successful ingestion result.
fn assert_all_rel_paths(input: &g3rs_fmt_types::G3RsFmtConfigChecksInput) {
    assert_eq!(
        input.rustfmt_rel_path, "rustfmt.toml",
        "rustfmt_rel_path should be the workspace-root-relative path"
    );
    assert_eq!(
        input.cargo_rel_path, "Cargo.toml",
        "cargo_rel_path should be the workspace-root-relative path"
    );
    assert_eq!(
        input.toolchain_rel_path, "rust-toolchain.toml",
        "toolchain_rel_path should be the workspace-root-relative path"
    );
}

/// Assert that parsed content matches the fixture constants.
fn assert_parsed_content(input: &g3rs_fmt_types::G3RsFmtConfigChecksInput) {
    assert_eq!(
        input.rustfmt.edition,
        Some(rustfmt_toml_parser::Edition::Edition2024),
        "parsed rustfmt should have edition 2024 from fixture content"
    );
    assert!(
        input.cargo.workspace.is_some(),
        "parsed Cargo.toml should have a [workspace] section from fixture content"
    );
    let toolchain_section = input
        .toolchain
        .toolchain
        .as_ref()
        .expect("parsed rust-toolchain.toml should have a [toolchain] section from fixture content");
    assert_eq!(
        toolchain_section.channel.as_deref(),
        Some("1.85.0"),
        "parsed toolchain should have channel 1.85.0 from fixture content"
    );
}

// ── Happy path ──────────────────────────────────────────────────────────

#[test]
fn ingests_all_three_files() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed when all three config files are present");

    assert_all_rel_paths(&input);
    assert_parsed_content(&input);
}

// ── Dot-prefixed .rustfmt.toml is NOT accepted ─────────────────────────

#[test]
fn dot_rustfmt_toml_is_not_accepted() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "ingestion should return RustfmtTomlNotFound when only .rustfmt.toml exists \
         (dot-prefixed variant is a policy violation, not an acceptable config)"
    );
}

#[test]
fn ignores_dot_rustfmt_toml_when_rustfmt_toml_exists() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join(".rustfmt.toml"), "edition = \"2021\"\n");
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed using rustfmt.toml even when .rustfmt.toml also exists");

    assert_all_rel_paths(&input);

    // Verify the correct file was parsed (2024 from rustfmt.toml, not 2021 from .rustfmt.toml).
    assert_eq!(
        input.rustfmt.edition,
        Some(rustfmt_toml_parser::Edition::Edition2024),
        "parsed edition should come from rustfmt.toml (2024), not .rustfmt.toml (2021)"
    );
}

// ── Missing file errors ─────────────────────────────────────────────────

#[test]
fn fails_when_rustfmt_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "ingestion should return RustfmtTomlNotFound when rustfmt.toml is absent"
    );
}

#[test]
fn fails_when_cargo_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::CargoTomlNotFound)),
        "ingestion should return CargoTomlNotFound when Cargo.toml is absent"
    );
}

#[test]
fn fails_when_toolchain_toml_is_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::ToolchainTomlNotFound)),
        "ingestion should return ToolchainTomlNotFound when rust-toolchain.toml is absent"
    );
}

#[test]
fn fails_when_all_files_are_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "ingestion should return RustfmtTomlNotFound as the first missing-file error \
         when no config files exist (rustfmt.toml is checked first)"
    );
}

// ── Parse failure errors ────────────────────────────────────────────────

#[test]
fn fails_on_malformed_rustfmt_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "{{{{not valid toml at all}}}}");
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    match result {
        Err(crate::IngestionError::ParseFailed { ref path, .. }) => {
            assert!(
                path.ends_with("rustfmt.toml"),
                "ParseFailed path should point to rustfmt.toml, got: {path:?}"
            );
        }
        ref other => panic!(
            "expected ParseFailed for rustfmt.toml, got: {other:?}"
        ),
    }
}

#[test]
fn fails_on_malformed_cargo_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    match result {
        Err(crate::IngestionError::ParseFailed { ref path, .. }) => {
            assert!(
                path.ends_with("Cargo.toml"),
                "ParseFailed path should point to Cargo.toml, got: {path:?}"
            );
        }
        ref other => panic!(
            "expected ParseFailed for Cargo.toml, got: {other:?}"
        ),
    }
}

#[test]
fn fails_on_malformed_toolchain_toml() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    match result {
        Err(crate::IngestionError::ParseFailed { ref path, .. }) => {
            assert!(
                path.ends_with("rust-toolchain.toml"),
                "ParseFailed path should point to rust-toolchain.toml, got: {path:?}"
            );
        }
        ref other => panic!(
            "expected ParseFailed for rust-toolchain.toml, got: {other:?}"
        ),
    }
}

// ── Gitignored but recovered files ──────────────────────────────────────

#[test]
fn ignored_but_recovered_rustfmt_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "rustfmt.toml\n");
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl).expect(
        "ingestion should succeed for a gitignored rustfmt.toml recovered by the crawl recovery phase",
    );

    assert_all_rel_paths(&input);
    assert_parsed_content(&input);
}

#[test]
fn ignored_but_recovered_cargo_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "Cargo.toml\n");
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl).expect(
        "ingestion should succeed for a gitignored Cargo.toml recovered by the crawl recovery phase",
    );

    assert_all_rel_paths(&input);
    assert_parsed_content(&input);
}

#[test]
fn ignored_but_recovered_toolchain_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "rust-toolchain.toml\n");
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl).expect(
        "ingestion should succeed for a gitignored rust-toolchain.toml recovered by the crawl recovery phase",
    );

    assert_all_rel_paths(&input);
    assert_parsed_content(&input);
}

// ── Empty file behavior ─────────────────────────────────────────────────

#[test]
fn empty_rustfmt_toml_is_valid() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "");
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("empty rustfmt.toml is valid TOML (all fields are optional)");

    assert_eq!(
        input.rustfmt.edition, None,
        "empty rustfmt.toml should have no edition set"
    );
}

// ── Error precedence (selection before parsing) ─────────────────────────

#[test]
fn malformed_rustfmt_plus_missing_cargo_returns_cargo_not_found() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "{{{{not valid toml}}}}");
    // No Cargo.toml written.
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    // Selection happens before parsing: rustfmt.toml passes selection (exists),
    // then Cargo.toml selection fails with CargoTomlNotFound. The malformed
    // rustfmt.toml is never parsed.
    assert!(
        matches!(result, Err(crate::IngestionError::CargoTomlNotFound)),
        "selection errors should take precedence over parse errors \
         because all three files are selected before any parsing occurs"
    );
}

// ── Error Display messages ──────────────────────────────────────────────

#[test]
fn error_display_includes_file_name() {
    let err = crate::IngestionError::RustfmtTomlNotFound;
    let msg = err.to_string();
    assert!(
        msg.contains("rustfmt.toml"),
        "RustfmtTomlNotFound display should mention rustfmt.toml, got: {msg}"
    );

    let err = crate::IngestionError::CargoTomlNotFound;
    let msg = err.to_string();
    assert!(
        msg.contains("Cargo.toml"),
        "CargoTomlNotFound display should mention Cargo.toml, got: {msg}"
    );

    let err = crate::IngestionError::ToolchainTomlNotFound;
    let msg = err.to_string();
    assert!(
        msg.contains("rust-toolchain.toml"),
        "ToolchainTomlNotFound display should mention rust-toolchain.toml, got: {msg}"
    );
}

// ── Realistic content (matches actual repo configs) ─────────────────────

#[test]
fn ingests_realistic_configs_with_all_check_relevant_fields() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), REALISTIC_RUSTFMT);
    write(root.join("Cargo.toml"), REALISTIC_CARGO);
    write(root.join("rust-toolchain.toml"), REALISTIC_TOOLCHAIN);

    let crawl = crawl(root);
    let input = crate::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed on realistic config content");

    assert_all_rel_paths(&input);

    // Verify all 8 settings that RS-FMT-CONFIG-01 reads from rustfmt.toml.
    assert_eq!(
        input.rustfmt.edition,
        Some(rustfmt_toml_parser::Edition::Edition2024),
        "realistic rustfmt should have edition 2024"
    );
    assert_eq!(
        input.rustfmt.style_edition,
        Some(rustfmt_toml_parser::StyleEdition::Edition2024),
        "realistic rustfmt should have style_edition 2024"
    );
    assert_eq!(
        input.rustfmt.max_width,
        Some(100),
        "realistic rustfmt should have max_width 100"
    );
    assert_eq!(
        input.rustfmt.tab_spaces,
        Some(4),
        "realistic rustfmt should have tab_spaces 4"
    );
    assert_eq!(
        input.rustfmt.use_field_init_shorthand,
        Some(true),
        "realistic rustfmt should have use_field_init_shorthand true"
    );
    assert_eq!(
        input.rustfmt.use_try_shorthand,
        Some(true),
        "realistic rustfmt should have use_try_shorthand true"
    );
    assert_eq!(
        input.rustfmt.reorder_imports,
        Some(true),
        "realistic rustfmt should have reorder_imports true"
    );
    assert_eq!(
        input.rustfmt.reorder_modules,
        Some(true),
        "realistic rustfmt should have reorder_modules true"
    );

    // Verify workspace.package.edition that RS-FMT-CONFIG-04 reads for
    // edition mismatch detection.
    let workspace = input
        .cargo
        .workspace
        .as_ref()
        .expect("realistic Cargo.toml should have a [workspace] section");
    let ws_package = workspace
        .package
        .as_ref()
        .expect("realistic Cargo.toml should have a [workspace.package] section");
    assert_eq!(
        ws_package.edition.as_deref(),
        Some("2024"),
        "realistic Cargo.toml workspace.package.edition should be 2024"
    );

    // Verify toolchain channel and components that RS-FMT-CONFIG-03 reads for
    // nightly key detection.
    let toolchain_section = input
        .toolchain
        .toolchain
        .as_ref()
        .expect("realistic rust-toolchain.toml should have a [toolchain] section");
    assert_eq!(
        toolchain_section.channel.as_deref(),
        Some("stable"),
        "realistic toolchain should have channel 'stable'"
    );
    assert_eq!(
        toolchain_section.components,
        vec!["clippy".to_owned(), "rustfmt".to_owned()],
        "realistic toolchain should have clippy and rustfmt components"
    );
}

// ── Subdirectory files are not selected ─────────────────────────────────

#[test]
fn rustfmt_toml_in_subdirectory_is_not_selected() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("subdir/rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "rustfmt.toml in a subdirectory should not be selected as a root config file"
    );
}

// ── Unreadable files ────────────────────────────────────────────────────

#[cfg(unix)]
#[test]
fn unreadable_rustfmt_toml_returns_unreadable_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    // Remove read permissions from rustfmt.toml.
    let rustfmt_path = root.join("rustfmt.toml");
    fs::set_permissions(&rustfmt_path, fs::Permissions::from_mode(0o000))
        .expect("should set permissions on rustfmt.toml");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    // Restore permissions so temp cleanup succeeds.
    let _ignore = fs::set_permissions(&rustfmt_path, fs::Permissions::from_mode(0o644));

    match result {
        Err(crate::IngestionError::Unreadable { ref path, .. }) => {
            assert!(
                path.ends_with("rustfmt.toml"),
                "Unreadable path should point to rustfmt.toml, got: {path:?}"
            );
        }
        // The crawl may also mark the file as unreadable via its `readable` flag,
        // which produces the same error variant from a different code path. Both
        // are correct — the important thing is it's `Unreadable` for rustfmt.toml.
        ref other => panic!(
            "expected Unreadable error for rustfmt.toml, got: {other:?}"
        ),
    }
}

#[cfg(unix)]
#[test]
fn unreadable_cargo_toml_returns_unreadable_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let cargo_path = root.join("Cargo.toml");
    fs::set_permissions(&cargo_path, fs::Permissions::from_mode(0o000))
        .expect("should set permissions on Cargo.toml");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let _ignore = fs::set_permissions(&cargo_path, fs::Permissions::from_mode(0o644));

    match result {
        Err(crate::IngestionError::Unreadable { ref path, .. }) => {
            assert!(
                path.ends_with("Cargo.toml"),
                "Unreadable path should point to Cargo.toml, got: {path:?}"
            );
        }
        ref other => panic!(
            "expected Unreadable error for Cargo.toml, got: {other:?}"
        ),
    }
}

#[cfg(unix)]
#[test]
fn unreadable_toolchain_toml_returns_unreadable_error() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let toolchain_path = root.join("rust-toolchain.toml");
    fs::set_permissions(&toolchain_path, fs::Permissions::from_mode(0o000))
        .expect("should set permissions on rust-toolchain.toml");

    let crawl = crawl(root);
    let result = crate::ingest_for_config_checks(&crawl);

    let _ignore = fs::set_permissions(&toolchain_path, fs::Permissions::from_mode(0o644));

    match result {
        Err(crate::IngestionError::Unreadable { ref path, .. }) => {
            assert!(
                path.ends_with("rust-toolchain.toml"),
                "Unreadable path should point to rust-toolchain.toml, got: {path:?}"
            );
        }
        ref other => panic!(
            "expected Unreadable error for rust-toolchain.toml, got: {other:?}"
        ),
    }
}
