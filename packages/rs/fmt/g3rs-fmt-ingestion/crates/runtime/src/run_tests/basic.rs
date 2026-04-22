use std::fs;

use tempfile::tempdir;

use super::helpers::{crawl, git_init, write};

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

/// Write all three required config files to the workspace root.
fn write_all_configs(root: &std::path::Path) {
    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);
}

// ── Happy path ──────────────────────────────────────────────────────────

#[test]
fn ingests_all_three_files() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed when all three config files are present");

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.toolchain_rel_path, "rust-toolchain.toml");
    let rustfmt = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => {
            panic!("expected parsed rustfmt content")
        }
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => {
            panic!("expected parsed rustfmt content")
        }
    };
    let cargo = match &input.cargo_state {
        g3rs_fmt_types::G3RsFmtCargoState::Parsed(cargo) => cargo,
        other => panic!("expected parsed Cargo.toml, got {other:?}"),
    };
    let toolchain = match &input.toolchain_state {
        g3rs_fmt_types::G3RsFmtToolchainState::Parsed(toolchain) => toolchain,
        other => panic!("expected parsed rust-toolchain.toml, got {other:?}"),
    };
    assert_eq!(rustfmt.edition, Some("2024".to_owned()));
    assert_eq!(cargo.edition.as_deref(), None);
    assert_eq!(toolchain.channel.as_deref(), Some("1.85.0"));
}

#[test]
fn dot_rustfmt_toml_is_accepted_when_root_rustfmt_toml_is_absent() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should accept root .rustfmt.toml when rustfmt.toml is absent");

    assert_eq!(input.rustfmt_rel_path, ".rustfmt.toml");
    assert_eq!(
        match &input.rustfmt_state {
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt.edition.clone(),
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => None,
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => None,
        },
        Some("2024".to_owned()),
        "parsed edition should come from .rustfmt.toml"
    );
}

#[test]
fn invalid_root_rustfmt_toml_is_preserved_for_config_rule_reporting() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "edition = [\n");
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve rustfmt parse failure for RS-FMT-CONFIG-01 reporting");

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
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
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed using rustfmt.toml even when .rustfmt.toml also exists");

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.toolchain_rel_path, "rust-toolchain.toml");

    // Verify the correct file was parsed (2024 from rustfmt.toml, not 2021 from .rustfmt.toml).
    assert_eq!(
        match &input.rustfmt_state {
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt.edition.clone(),
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => None,
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => None,
        },
        Some("2024".to_owned()),
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
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "ingestion should return RustfmtTomlNotFound when rustfmt.toml is absent"
    );
}

#[test]
fn preserves_missing_cargo_toml_for_config_blockers() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve missing Cargo.toml for config rules");

    assert!(matches!(
        input.cargo_state,
        g3rs_fmt_types::G3RsFmtCargoState::Missing
    ));
}

#[test]
fn preserves_missing_toolchain_toml_for_config_blockers() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve missing rust-toolchain.toml for config rules");

    assert!(matches!(
        input.toolchain_state,
        g3rs_fmt_types::G3RsFmtToolchainState::Missing
    ));
}

#[test]
fn fails_when_all_files_are_missing() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    let crawl = crawl(root);
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "ingestion should return RustfmtTomlNotFound as the first missing-file error \
         when no config files exist (rustfmt.toml is checked first)"
    );
}

// ── Parse failure errors ────────────────────────────────────────────────

#[test]
fn preserves_malformed_rustfmt_toml_for_config_rule_reporting() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "{{{{not valid toml at all}}}}");
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve malformed rustfmt.toml");

    assert!(matches!(
        input.rustfmt_state,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError
    ));
}

#[test]
fn preserves_malformed_cargo_toml_for_config_rule_reporting() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), "{{{{not valid toml}}}}");
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve malformed Cargo.toml");

    assert!(matches!(
        input.cargo_state,
        g3rs_fmt_types::G3RsFmtCargoState::ParseError
    ));
}

#[test]
fn preserves_malformed_toolchain_toml_for_config_rule_reporting() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), "{{{{not valid toml}}}}");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve malformed rust-toolchain.toml");

    assert!(matches!(
        input.toolchain_state,
        g3rs_fmt_types::G3RsFmtToolchainState::ParseError
    ));
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
    let input = crate::run::ingest_for_config_checks(&crawl).expect(
        "ingestion should succeed for a gitignored rustfmt.toml recovered by the crawl recovery phase",
    );

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.toolchain_rel_path, "rust-toolchain.toml");
    let rustfmt = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt,
        other => panic!("expected parsed rustfmt content, got {other:?}"),
    };
    let cargo = match &input.cargo_state {
        g3rs_fmt_types::G3RsFmtCargoState::Parsed(cargo) => cargo,
        other => panic!("expected parsed Cargo.toml, got {other:?}"),
    };
    let toolchain = match &input.toolchain_state {
        g3rs_fmt_types::G3RsFmtToolchainState::Parsed(toolchain) => toolchain,
        other => panic!("expected parsed rust-toolchain.toml, got {other:?}"),
    };
    assert_eq!(rustfmt.edition, Some("2024".to_owned()));
    assert_eq!(cargo.edition.as_deref(), None);
    assert_eq!(toolchain.channel.as_deref(), Some("1.85.0"));
}

#[test]
fn ignored_but_recovered_cargo_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "Cargo.toml\n");
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl).expect(
        "ingestion should succeed for a gitignored Cargo.toml recovered by the crawl recovery phase",
    );

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.toolchain_rel_path, "rust-toolchain.toml");
    let rustfmt = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt,
        other => panic!("expected parsed rustfmt content, got {other:?}"),
    };
    let cargo = match &input.cargo_state {
        g3rs_fmt_types::G3RsFmtCargoState::Parsed(cargo) => cargo,
        other => panic!("expected parsed Cargo.toml, got {other:?}"),
    };
    let toolchain = match &input.toolchain_state {
        g3rs_fmt_types::G3RsFmtToolchainState::Parsed(toolchain) => toolchain,
        other => panic!("expected parsed rust-toolchain.toml, got {other:?}"),
    };
    assert_eq!(rustfmt.edition, Some("2024".to_owned()));
    assert_eq!(cargo.edition.as_deref(), None);
    assert_eq!(toolchain.channel.as_deref(), Some("1.85.0"));
}

#[test]
fn ignored_but_recovered_toolchain_toml_is_ingested() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".gitignore"), "rust-toolchain.toml\n");
    write_all_configs(root);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl).expect(
        "ingestion should succeed for a gitignored rust-toolchain.toml recovered by the crawl recovery phase",
    );

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.toolchain_rel_path, "rust-toolchain.toml");
    let rustfmt = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt,
        other => panic!("expected parsed rustfmt content, got {other:?}"),
    };
    let cargo = match &input.cargo_state {
        g3rs_fmt_types::G3RsFmtCargoState::Parsed(cargo) => cargo,
        other => panic!("expected parsed Cargo.toml, got {other:?}"),
    };
    let toolchain = match &input.toolchain_state {
        g3rs_fmt_types::G3RsFmtToolchainState::Parsed(toolchain) => toolchain,
        other => panic!("expected parsed rust-toolchain.toml, got {other:?}"),
    };
    assert_eq!(rustfmt.edition, Some("2024".to_owned()));
    assert_eq!(cargo.edition.as_deref(), None);
    assert_eq!(toolchain.channel.as_deref(), Some("1.85.0"));
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
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("empty rustfmt.toml is valid TOML (all fields are optional)");

    assert_eq!(
        match &input.rustfmt_state {
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt.edition.clone(),
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => {
                panic!("empty rustfmt.toml should parse")
            }
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => {
                panic!("empty rustfmt.toml should parse")
            }
        },
        None,
        "empty rustfmt.toml should have no edition set"
    );
}

// ── Error precedence (selection before parsing) ─────────────────────────

#[test]
fn malformed_rustfmt_plus_missing_cargo_preserves_both_states() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join("rustfmt.toml"), "{{{{not valid toml}}}}");
    // No Cargo.toml written.
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("config ingestion should preserve rule-owned blocker states");

    assert!(
        matches!(
            input.rustfmt_state,
            g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError
        ),
        "invalid rustfmt.toml should stay visible to RS-FMT-CONFIG-01"
    );
    assert!(
        matches!(
            input.cargo_state,
            g3rs_fmt_types::G3RsFmtCargoState::Missing
        ),
        "missing Cargo.toml should stay visible to RS-FMT-CONFIG-04"
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
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should succeed on realistic config content");

    assert_eq!(input.rustfmt_rel_path, "rustfmt.toml");
    assert_eq!(input.cargo_rel_path, "Cargo.toml");
    assert_eq!(input.toolchain_rel_path, "rust-toolchain.toml");

    // Verify all 8 settings that RS-FMT-CONFIG-01 reads from rustfmt.toml.
    let rustfmt = match &input.rustfmt_state {
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Parsed(rustfmt) => rustfmt,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable => {
            panic!("realistic rustfmt should parse")
        }
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::ParseError => {
            panic!("realistic rustfmt should parse")
        }
    };
    assert_eq!(
        rustfmt.edition,
        Some("2024".to_owned()),
        "realistic rustfmt should have edition 2024"
    );
    assert_eq!(
        rustfmt.style_edition,
        Some("2024".to_owned()),
        "realistic rustfmt should have style_edition 2024"
    );
    assert_eq!(
        rustfmt.max_width,
        Some(100),
        "realistic rustfmt should have max_width 100"
    );
    assert_eq!(
        rustfmt.tab_spaces,
        Some(4),
        "realistic rustfmt should have tab_spaces 4"
    );
    assert_eq!(
        rustfmt.use_field_init_shorthand,
        Some(true),
        "realistic rustfmt should have use_field_init_shorthand true"
    );
    assert_eq!(
        rustfmt.use_try_shorthand,
        Some(true),
        "realistic rustfmt should have use_try_shorthand true"
    );
    assert_eq!(
        rustfmt.reorder_imports,
        Some(true),
        "realistic rustfmt should have reorder_imports true"
    );
    assert_eq!(
        rustfmt.reorder_modules,
        Some(true),
        "realistic rustfmt should have reorder_modules true"
    );

    // Verify workspace.package.edition that RS-FMT-CONFIG-04 reads for
    // edition mismatch detection.
    let cargo = match &input.cargo_state {
        g3rs_fmt_types::G3RsFmtCargoState::Parsed(cargo) => cargo,
        other => panic!("realistic Cargo.toml should parse, got {other:?}"),
    };
    assert_eq!(
        cargo.edition.as_deref(),
        Some("2024"),
        "realistic Cargo.toml workspace.package.edition should be 2024"
    );

    // Verify toolchain channel and components that RS-FMT-CONFIG-03 reads for
    // nightly key detection.
    let toolchain = match &input.toolchain_state {
        g3rs_fmt_types::G3RsFmtToolchainState::Parsed(toolchain) => toolchain,
        other => panic!("realistic toolchain should parse, got {other:?}"),
    };
    assert_eq!(
        toolchain.channel.as_deref(),
        Some("stable"),
        "realistic toolchain should have channel 'stable'"
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
    let result = crate::run::ingest_for_config_checks(&crawl);

    assert!(
        matches!(result, Err(crate::IngestionError::RustfmtTomlNotFound)),
        "rustfmt.toml in a subdirectory should not be selected as a root config file"
    );
}

// ── Unreadable files ────────────────────────────────────────────────────

#[cfg(unix)]
#[test]
fn unreadable_rustfmt_toml_becomes_parse_error_state() {
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
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve unreadable rustfmt.toml as state");

    // Restore permissions so temp cleanup succeeds.
    let _ignore = fs::set_permissions(&rustfmt_path, fs::Permissions::from_mode(0o644));

    assert!(matches!(
        input.rustfmt_state,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable
    ));
}

#[cfg(unix)]
#[test]
fn unreadable_cargo_toml_becomes_parse_error_state() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let cargo_path = root.join("Cargo.toml");
    fs::set_permissions(&cargo_path, fs::Permissions::from_mode(0o000))
        .expect("should set permissions on Cargo.toml");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve unreadable Cargo.toml as state");

    let _ignore = fs::set_permissions(&cargo_path, fs::Permissions::from_mode(0o644));

    assert!(matches!(
        input.cargo_state,
        g3rs_fmt_types::G3RsFmtCargoState::Unreadable
    ));
}

#[cfg(unix)]
#[test]
fn unreadable_toolchain_toml_becomes_parse_error_state() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let toolchain_path = root.join("rust-toolchain.toml");
    fs::set_permissions(&toolchain_path, fs::Permissions::from_mode(0o000))
        .expect("should set permissions on rust-toolchain.toml");

    let crawl = crawl(root);
    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve unreadable rust-toolchain.toml as state");

    let _ignore = fs::set_permissions(&toolchain_path, fs::Permissions::from_mode(0o644));

    assert!(matches!(
        input.toolchain_state,
        g3rs_fmt_types::G3RsFmtToolchainState::Unreadable
    ));
}

#[test]
fn deleted_after_crawl_rustfmt_toml_becomes_parse_error_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let rustfmt_path = root.join("rustfmt.toml");
    let crawl = crawl(root);
    fs::remove_file(&rustfmt_path).expect("should remove rustfmt.toml after crawl");

    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve deleted-after-crawl rustfmt.toml as state");

    assert!(matches!(
        input.rustfmt_state,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable
    ));
}

#[test]
fn deleted_after_crawl_cargo_toml_becomes_parse_error_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let cargo_path = root.join("Cargo.toml");
    let crawl = crawl(root);
    fs::remove_file(&cargo_path).expect("should remove Cargo.toml after crawl");

    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve deleted-after-crawl Cargo.toml as state");

    assert!(matches!(
        input.cargo_state,
        g3rs_fmt_types::G3RsFmtCargoState::Unreadable
    ));
}

#[test]
fn deleted_after_crawl_toolchain_toml_becomes_parse_error_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);
    write_all_configs(root);

    let toolchain_path = root.join("rust-toolchain.toml");
    let crawl = crawl(root);
    fs::remove_file(&toolchain_path).expect("should remove rust-toolchain.toml after crawl");

    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve deleted-after-crawl rust-toolchain.toml as state");

    assert!(matches!(
        input.toolchain_state,
        g3rs_fmt_types::G3RsFmtToolchainState::Unreadable
    ));
}

#[test]
fn deleted_after_crawl_dot_rustfmt_toml_becomes_parse_error_state() {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    let root = temp.path();
    git_init(root);

    write(root.join(".rustfmt.toml"), RUSTFMT_CONTENT);
    write(root.join("Cargo.toml"), CARGO_CONTENT);
    write(root.join("rust-toolchain.toml"), TOOLCHAIN_CONTENT);

    let dot_rustfmt_path = root.join(".rustfmt.toml");
    let crawl = crawl(root);
    fs::remove_file(&dot_rustfmt_path).expect("should remove .rustfmt.toml after crawl");

    let input = crate::run::ingest_for_config_checks(&crawl)
        .expect("ingestion should preserve deleted-after-crawl .rustfmt.toml as state");

    assert_eq!(input.rustfmt_rel_path, ".rustfmt.toml");
    assert!(matches!(
        input.rustfmt_state,
        g3rs_fmt_types::G3RsFmtRustfmtConfigState::Unreadable
    ));
}
