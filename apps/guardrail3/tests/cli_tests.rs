//! Integration tests for guardrail3 CLI commands.
//!
//! These tests invoke the compiled binary and check exit codes + output
//! to kill known surviving mutants in the CLI layer.
use garde as _;

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use glob as _;
use guardrail3 as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

use std::process::Command;

#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
fn guardrail3() -> Command {
    Command::new(env!("CARGO_BIN_EXE_guardrail3"))
}

const fn project_root() -> &'static str {
    env!("CARGO_MANIFEST_DIR")
}

/// Workspace root is two directories up from the crate manifest dir (apps/guardrail3/ -> .)
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("parent of crate dir")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

// ---- Format matching (main.rs lines 62,86,114: json arm; 87,115: md arm) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_json_format_produces_valid_json() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains('{'),
        "JSON format should produce JSON output"
    );
    assert!(
        stdout.contains("\"project\""),
        "JSON output should contain project field"
    );
    assert!(
        !stdout.contains("\x1b["),
        "JSON output should not contain ANSI escape codes"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_md_format_produces_markdown() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "md", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("# Guardrail3 Validation Report"),
        "Markdown format should produce markdown header"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_markdown_alias_produces_markdown() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "markdown", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("# Guardrail3 Validation Report"),
        "markdown alias should also produce markdown output"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_text_is_default_format() {
    let out = guardrail3()
        .args(["rs", "validate", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        !stdout.starts_with('{'),
        "Default format should not be JSON"
    );
    assert!(
        !stdout.starts_with("# Guardrail3"),
        "Default format should not be markdown"
    );
}

// ---- Exit code logic (main.rs lines 66,90,118: > 0) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_exit_code_nonzero_when_errors() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    if stdout.contains("\"errors\": 0") {
        assert!(out.status.success(), "Should exit 0 when no errors");
    } else {
        assert!(
            !out.status.success(),
            "Should exit non-zero when errors exist"
        );
    }
}

// ---- rs validate with json/md format (main.rs lines 62,87) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_rs_validate_json_format() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains('{'),
        "rs validate --format json should produce JSON"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_rs_validate_md_format() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "md", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("# Guardrail3"),
        "rs validate --format md should produce markdown"
    );
}

// ---- hooks validate with json/md (main.rs lines 114,115) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_hooks_validate_json_format() {
    let out = guardrail3()
        .args(["rs", "hooks-validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains('{'),
        "hooks validate --format json should produce JSON"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_hooks_validate_md_format() {
    let out = guardrail3()
        .args(["rs", "hooks-validate", "--format", "md", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("# Guardrail3"),
        "hooks validate --format md should produce markdown"
    );
}

// ---- Domain filtering (main.rs lines 130-135: domains_from_args) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_code_domain_only() {
    let out = guardrail3()
        .args([
            "rs",
            "validate",
            "--format",
            "json",
            "--code",
            project_root(),
        ])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"project\""),
        "--code flag should still produce report"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_architecture_domain_only() {
    let out = guardrail3()
        .args([
            "rs",
            "validate",
            "--format",
            "json",
            "--architecture",
            project_root(),
        ])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("\"project\""),
        "--architecture flag should produce report"
    );
}

// ---- discover.rs lines 28,130: negation in detect_project/detect_rust ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_detects_rust_project() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Rust"),
        "Should detect Rust stack on guardrail3 project"
    );
}

// ---- commands/check.rs lines 20,32: == vs != ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_check_on_self() {
    let ws = workspace_root();
    let out = guardrail3()
        .args(["rs", "check"])
        .arg(&ws)
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let has_current = stdout.contains("All generated files are current.");
    let has_stale = stdout.contains("STALE:") || stdout.contains("MISSING:");
    assert!(
        has_current || has_stale,
        "check command should report file status, got: {stdout}"
    );

    if has_stale {
        assert!(
            !out.status.success(),
            "check should exit non-zero when files are stale"
        );
    }
    if has_current {
        assert!(
            out.status.success(),
            "check should exit zero when files are current"
        );
    }
}

// ---- commands/diff.rs lines 20,55,61: comparison and display logic ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_diff_on_self() {
    let ws = workspace_root();
    let out = guardrail3()
        .args(["rs", "diff"])
        .arg(&ws)
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let no_changes = stdout.contains("No changes");
    let has_diff = stdout.contains("---") && stdout.contains("+++");
    assert!(
        no_changes || has_diff,
        "diff should show diffs or no-changes message, got: {stdout}"
    );

    if has_diff {
        assert!(
            !out.status.success(),
            "diff should exit non-zero when diffs exist"
        );
    }
    if no_changes {
        assert!(
            out.status.success(),
            "diff should exit zero when no changes"
        );
    }
}

// ---- commands/init.rs lines 13,61,72,83,100,112,132 ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cli_init_service_profile() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let out = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "init should succeed in empty dir: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Initialized Rust guardrail3"),
        "Should print init message"
    );
    assert!(stdout.contains("service"), "Should mention service profile");

    assert!(
        tmp.path().join("guardrail3.toml").exists(),
        "guardrail3.toml should be created"
    );
    // local/*.toml and release files are created by `generate`, not `init`
    assert!(
        stdout.contains("guardrail3 rs generate"),
        "Should tell user to run generate next"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cli_init_library_profile_differs_from_service() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let out = guardrail3()
        .args(["rs", "init", "--profile", "library", path])
        .output()
        .expect("failed to run");

    assert!(out.status.success(), "init library should succeed");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("library"), "Should mention library profile");

    assert!(
        !tmp.path().join("release-plz.toml").exists(),
        "Library profile should NOT create release-plz.toml"
    );

    let config =
        std::fs::read_to_string(tmp.path().join("guardrail3.toml")).expect("should read config");
    assert!(
        config.contains("\"library\""),
        "Config should contain library profile name"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_init_refuses_overwrite_without_force() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    let out = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "init without --force should fail when config exists"
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("already exists"),
        "Should report config already exists"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cli_init_force_overwrites() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    let out = guardrail3()
        .args(["rs", "init", "--profile", "library", "--force", path])
        .output()
        .expect("failed to run");

    assert!(out.status.success(), "init with --force should succeed");

    let config =
        std::fs::read_to_string(tmp.path().join("guardrail3.toml")).expect("should read config");
    assert!(
        config.contains("\"library\""),
        "Force init should overwrite with new profile"
    );
}

// ---- commands/generate.rs lines 303,321,328,353,388 ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cli_generate_produces_files() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    std::fs::write(
        tmp.path().join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml");

    let out = guardrail3()
        .args(["rs", "generate", path])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("wrote:"),
        "generate should report written files"
    );

    assert!(
        tmp.path().join("clippy.toml").exists(),
        "generate should create clippy.toml"
    );
    assert!(
        tmp.path().join("deny.toml").exists(),
        "generate should create deny.toml"
    );
    assert!(
        tmp.path().join("rustfmt.toml").exists(),
        "generate should create rustfmt.toml"
    );
    // Hooks are NOT created by rs generate — use rs hooks-install separately
    // assert!(tmp.path().join(".githooks/pre-commit").exists());
}

// ---- commands/validate.rs lines 25-30,79,80,85,96,123 ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_format_json_in_combined_validate() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "JSON output should be valid JSON");
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_format_md_in_combined_validate() {
    let out = guardrail3()
        .args(["rs", "validate", "--format", "md", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.starts_with("# "),
        "Markdown output should start with heading"
    );
}

// ---- commands/modules_cmd.rs lines 5,13,14 ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_list_modules_has_output() {
    let out = guardrail3()
        .args(["rs", "list-modules"])
        .output()
        .expect("failed to run");

    assert!(out.status.success(), "list-modules should succeed");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("==="),
        "list-modules should print category headers"
    );
    assert!(
        stdout.contains("clippy"),
        "list-modules should include clippy modules"
    );

    let category_count = stdout.matches("===").count();
    assert!(
        category_count > 1,
        "list-modules should have multiple categories, got {category_count}"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_show_module_prints_content() {
    let out = guardrail3()
        .args(["rs", "show-module", "clippy/methods/env-vars"])
        .output()
        .expect("failed to run");

    assert!(out.status.success(), "show-module should succeed");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("# Module:"),
        "show-module should print module header"
    );
    assert!(
        stdout.contains("env"),
        "show-module should print module content"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_show_module_nonexistent_fails() {
    let out = guardrail3()
        .args(["rs", "show-module", "nonexistent/module"])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "show-module for nonexistent module should fail"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("not found"),
        "Should report module not found"
    );
}

// ---- ts validate on a non-TS project ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_ts_validate_on_rust_project() {
    let out = guardrail3()
        .args(["ts", "validate", "--format", "json", project_root()])
        .output()
        .expect("failed to run");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains('{'),
        "ts validate should produce JSON output even on non-TS project"
    );
}

// ---- check/diff on project without guardrail3.toml ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_check_without_config_fails() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let out = guardrail3()
        .args(["rs", "check", path])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "check without guardrail3.toml should fail"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("guardrail3.toml"),
        "Should mention missing config"
    );
}

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_diff_without_config_fails() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let out = guardrail3()
        .args(["rs", "diff", path])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "diff without guardrail3.toml should fail"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("guardrail3.toml"),
        "Should mention missing config"
    );
}

// ---- generate without config ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_generate_without_config_fails() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let out = guardrail3()
        .args(["rs", "generate", path])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "generate without guardrail3.toml should fail"
    );
}

// ---- validate on nonexistent path ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_validate_nonexistent_path_fails() {
    let out = guardrail3()
        .args(["rs", "validate", "/nonexistent/path/xyz"])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "validate on nonexistent path should fail"
    );

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("cannot resolve path"),
        "Should report path resolution error"
    );
}

// ---- check after generate produces "all current" (check.rs line 32) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_check_after_generate_is_current() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    let _ = guardrail3()
        .args(["rs", "generate", path])
        .output()
        .expect("failed to run");

    let out = guardrail3()
        .args(["rs", "check", path])
        .output()
        .expect("failed to run");

    assert!(
        out.status.success(),
        "check after generate should succeed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("All generated files are current"),
        "check should report all files current after generate"
    );
}

// ---- diff after generate shows no changes ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_diff_after_generate_no_changes() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    let _ = guardrail3()
        .args(["rs", "generate", path])
        .output()
        .expect("failed to run");

    let out = guardrail3()
        .args(["rs", "diff", path])
        .output()
        .expect("failed to run");

    assert!(out.status.success(), "diff after generate should succeed");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("No changes"),
        "diff should report no changes after generate"
    );
}

// ---- check detects stale files (check.rs line 20: actual != expected) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cli_check_detects_stale_after_tampering() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");
    let _ = guardrail3()
        .args(["rs", "generate", path])
        .output()
        .expect("failed to run");

    std::fs::write(tmp.path().join("rustfmt.toml"), "# tampered\n").expect("tamper file");

    let out = guardrail3()
        .args(["rs", "check", path])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "check should fail when files are stale"
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("STALE:"), "check should report STALE file");
}

// ---- diff detects diffs after tampering (diff.rs lines 20,55,61) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn cli_diff_shows_diff_after_tampering() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");
    let _ = guardrail3()
        .args(["rs", "generate", path])
        .output()
        .expect("failed to run");

    std::fs::write(tmp.path().join("rustfmt.toml"), "# tampered\n").expect("tamper file");

    let out = guardrail3()
        .args(["rs", "diff", path])
        .output()
        .expect("failed to run");

    assert!(!out.status.success(), "diff should fail when files differ");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("---"), "diff should show --- marker");
    assert!(stdout.contains("+++"), "diff should show +++ marker");
    assert!(
        stdout.contains("rustfmt.toml"),
        "diff should mention the tampered file"
    );
}

// ---- diff shows new file when file missing (diff.rs line 55: /dev/null) ----

#[test]
#[allow(clippy::expect_used)] // reason: test — panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command
fn cli_diff_shows_new_file_when_missing() {
    let tmp = tempfile::tempdir().expect("failed to create temp dir");
    let path = tmp.path().to_str().expect("non-utf8 path");

    let _ = guardrail3()
        .args(["rs", "init", "--profile", "service", path])
        .output()
        .expect("failed to run");

    let out = guardrail3()
        .args(["rs", "diff", path])
        .output()
        .expect("failed to run");

    assert!(
        !out.status.success(),
        "diff should fail when generated files are missing"
    );

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("/dev/null"),
        "diff should show /dev/null for missing files"
    );
    assert!(
        stdout.contains("(new file)"),
        "diff should indicate new file"
    );
}
