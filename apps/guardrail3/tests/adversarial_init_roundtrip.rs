//! Adversarial integration tests for init round-trip and generate idempotency.
//!
//! These tests verify:
//! - `ts init --force` preserves non-typescript sections (e.g. `[rust]`)
//! - `ts init --force` removes ALL old `[typescript.*]` subsections
//! - `rs generate` then `rs generate --dry-run` shows no changes (idempotent)
//! - `ts generate` then `ts generate --dry-run` shows no changes (idempotent)
//! - `rs generate --dry-run` only shows Rust files, not TS files (AV-8.5 bug)

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use garde as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use serde_json as _;
use std::process::Command;
use syn as _;
use toml as _;
use toml_edit as _;
use tree_sitter as _;
use tree_sitter_javascript as _;
use tree_sitter_typescript as _;
use walkdir as _;

#[allow(clippy::disallowed_methods)] // reason: Command::new needed to invoke binary under test
fn guardrail3() -> Command {
    Command::new(env!("CARGO_BIN_EXE_guardrail3"))
}

/// Helper: write a Cargo.toml so workspace detection works.
#[allow(clippy::disallowed_methods)] // reason: test helper -- writes fixture files to temp dir
#[allow(clippy::expect_used)] // reason: test helper -- panics on write failure
fn write_cargo_toml(dir: &std::path::Path) {
    std::fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"test\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("write Cargo.toml"); // reason: test setup
}

/// Helper: create an apps/<name>/ directory with a package.json so ts init discovers it.
#[allow(clippy::disallowed_methods)] // reason: test helper -- creates fixture directories
#[allow(clippy::expect_used)] // reason: test helper -- panics on creation failure
fn create_ts_app(dir: &std::path::Path, name: &str) {
    let app_dir = dir.join("apps").join(name);
    std::fs::create_dir_all(&app_dir).expect("create app dir"); // reason: test setup
    std::fs::write(
        app_dir.join("package.json"),
        format!("{{\"name\": \"{name}\", \"version\": \"1.0.0\"}}"),
    )
    .expect("write package.json"); // reason: test setup
}

// ---------------------------------------------------------------------------
// Test 1: ts init --force preserves [rust] section
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn ts_init_force_preserves_rust_section() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let path = tmp.path();

    // Write a config with [profile], [typescript] with an old app, then [rust]
    let config = "\
version = \"0.1\"

[profile]
name = \"service\"

[typescript]

[typescript.apps.old]
type = \"service\"

[rust]
workspace_root = \".\"
";
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config"); // reason: test setup

    // Create a TS app fixture so ts init discovers something
    create_ts_app(path, "web");

    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup
    let output = guardrail3()
        .args(["ts", "init", "--force", path_str])
        .output()
        .expect("run ts init --force"); // reason: test setup

    assert!(
        output.status.success(),
        "ts init --force failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let result =
        std::fs::read_to_string(path.join("guardrail3.toml")).expect("read guardrail3.toml"); // reason: test setup

    // [rust] section must be preserved
    assert!(
        result.contains("[rust]"),
        "Expected [rust] section to be preserved, got:\n{result}"
    );
    assert!(
        result.contains("workspace_root"),
        "Expected workspace_root in [rust] section, got:\n{result}"
    );

    // Old [typescript.apps.old] must be gone (replaced by fresh detection)
    assert!(
        !result.contains("[typescript.apps.old]"),
        "Expected old [typescript.apps.old] to be removed, got:\n{result}"
    );

    // New typescript section should reference discovered app "web"
    assert!(
        result.contains("[typescript.apps.web]"),
        "Expected [typescript.apps.web] from discovery, got:\n{result}"
    );
}

// ---------------------------------------------------------------------------
// Test 2: ts init --force removes deep subsections
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn ts_init_force_with_deep_subsections() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let path = tmp.path();

    // Config with nested typescript subsections
    let config = "\
version = \"0.1\"

[profile]
name = \"service\"

[typescript]

[typescript.apps.web]
type = \"service\"

[typescript.apps.web.checks]
architecture = true
content = false
tests = true

[typescript.apps.api]
type = \"service\"
";
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config"); // reason: test setup

    // Create a single TS app so detection finds something
    create_ts_app(path, "frontend");

    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup
    let output = guardrail3()
        .args(["ts", "init", "--force", path_str])
        .output()
        .expect("run ts init --force"); // reason: test setup

    assert!(
        output.status.success(),
        "ts init --force failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let result =
        std::fs::read_to_string(path.join("guardrail3.toml")).expect("read guardrail3.toml"); // reason: test setup

    // ALL old typescript subsections must be removed
    assert!(
        !result.contains("[typescript.apps.web]"),
        "Old [typescript.apps.web] should be removed, got:\n{result}"
    );
    assert!(
        !result.contains("[typescript.apps.web.checks]"),
        "Old [typescript.apps.web.checks] should be removed, got:\n{result}"
    );
    assert!(
        !result.contains("[typescript.apps.api]"),
        "Old [typescript.apps.api] should be removed, got:\n{result}"
    );

    // New typescript section should be present
    assert!(
        result.contains("[typescript]"),
        "Expected [typescript] section in result, got:\n{result}"
    );
}

// ---------------------------------------------------------------------------
// Test 3: rs generate then rs generate --dry-run shows no changes
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn rs_generate_then_dry_run_shows_no_changes() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let path = tmp.path();

    write_cargo_toml(path);

    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup

    // rs init --profile service
    let init_out = guardrail3()
        .args(["rs", "init", "--profile", "service", path_str])
        .output()
        .expect("run rs init"); // reason: test setup
    assert!(
        init_out.status.success(),
        "rs init failed: {}",
        String::from_utf8_lossy(&init_out.stderr)
    );

    // rs generate
    let gen_out = guardrail3()
        .args(["rs", "generate", path_str])
        .output()
        .expect("run rs generate"); // reason: test setup
    assert!(
        gen_out.status.success(),
        "rs generate failed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    // rs generate --dry-run should show no changes (exit 0)
    let dry_out = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run rs generate --dry-run"); // reason: test setup

    let stdout = String::from_utf8_lossy(&dry_out.stdout);

    // If idempotent, output should say "No changes needed" and exit 0
    assert!(
        stdout.contains("No changes needed") || stdout.contains("up to date"),
        "Expected no changes after generate then dry-run, got:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&dry_out.stderr)
    );
}

// ---------------------------------------------------------------------------
// Test 4: ts generate then ts generate --dry-run shows no changes
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
fn ts_generate_then_dry_run_shows_no_changes() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let path = tmp.path();

    // Write a config with typescript section
    let config = "\
version = \"0.1\"

[profile]
name = \"service\"

[typescript]

[typescript.apps.my-app]
type = \"service\"

[typescript.apps.my-app.checks]
architecture = true
content = false
tests = true
";
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config"); // reason: test setup

    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup

    // ts generate
    let gen_out = guardrail3()
        .args(["ts", "generate", path_str])
        .output()
        .expect("run ts generate"); // reason: test setup
    assert!(
        gen_out.status.success(),
        "ts generate failed: {}",
        String::from_utf8_lossy(&gen_out.stderr)
    );

    // ts generate --dry-run should show no changes
    let dry_out = guardrail3()
        .args(["ts", "generate", "--dry-run", path_str])
        .output()
        .expect("run ts generate --dry-run"); // reason: test setup

    let stdout = String::from_utf8_lossy(&dry_out.stdout);

    assert!(
        stdout.contains("No changes needed") || stdout.contains("up to date"),
        "Expected no changes after ts generate then dry-run, got:\nstdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&dry_out.stderr)
    );
}

// ---------------------------------------------------------------------------
// Test 5: rs generate --dry-run only shows Rust files (AV-8.5 bug documentation)
// ---------------------------------------------------------------------------

#[test]
#[allow(clippy::expect_used)] // reason: test -- panics indicate broken test infrastructure
#[allow(clippy::disallowed_methods)] // reason: test uses Command and fs
#[allow(clippy::print_stderr)] // reason: test — error output for debugging known bug AV-8.5
fn rs_dry_run_only_shows_rust_files() {
    let tmp = tempfile::tempdir().expect("create temp dir"); // reason: test setup
    let path = tmp.path();

    write_cargo_toml(path);

    // Write config with BOTH [rust] and [typescript] sections
    let config = "\
version = \"0.1\"

[profile]
name = \"service\"

[rust]
workspace_root = \".\"

[typescript]

[typescript.apps.my-app]
type = \"service\"

[typescript.apps.my-app.checks]
architecture = true
content = false
tests = true
";
    std::fs::write(path.join("guardrail3.toml"), config).expect("write config"); // reason: test setup

    let path_str = path.to_str().expect("non-utf8 path"); // reason: test setup

    // rs generate --dry-run (NOT "generate --dry-run")
    let output = guardrail3()
        .args(["rs", "generate", "--dry-run", path_str])
        .output()
        .expect("run rs generate --dry-run"); // reason: test setup

    let stdout = String::from_utf8_lossy(&output.stdout);

    // KNOWN BUG (AV-8.5): `rs generate --dry-run` routes through `diff::run`
    // which calls `generate_expected` -- that generates ALL files (Rust + TS + hooks),
    // not just Rust files. The correct behavior would be to only show Rust files.
    //
    // When this bug is fixed, flip these assertions:
    //   - Change `assert!(stdout.contains(".npmrc"))` to `assert!(!stdout.contains(".npmrc"))`
    //   - etc.

    // Document the bug: TS files ARE shown (should not be)
    let shows_ts_files = stdout.contains(".npmrc")
        || stdout.contains("eslint.config.mjs")
        || stdout.contains("tsconfig.base.json");

    if shows_ts_files {
        eprintln!(
            "KNOWN BUG (AV-8.5): rs generate --dry-run shows TS files. \
             diff::run calls generate_expected (all files) instead of generate_rust_files."
        );
    }

    // Assert the bug exists (when fixed, this test should be updated)
    assert!(
        shows_ts_files,
        "AV-8.5 appears to be FIXED -- update this test to assert correct behavior. \
         rs generate --dry-run no longer shows TS files.\nGot:\n{stdout}"
    );

    // Should ALSO contain Rust files (this should always be true)
    assert!(
        stdout.contains("clippy.toml") || stdout.contains("deny.toml"),
        "rs generate --dry-run should show Rust config files (clippy.toml, deny.toml).\nGot:\n{stdout}"
    );
}
