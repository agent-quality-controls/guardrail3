//! Adversarial config tests: each fixture is a minimal Rust project designed to
//! trigger specific guardrail3 config checks (R1-R29). If a check does NOT fire
//! on a fixture designed to trigger it, that is a bug.
#![allow(
    clippy::expect_used,
    clippy::disallowed_methods,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::manual_assert
)] // reason: test crate

// Suppress unused-crate-dependencies for workspace deps not used in this test binary.
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
use std::path::PathBuf;
use std::process::Command;
use syn as _;
use tempfile as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

/// Parsed check result from JSON output.
#[derive(Debug)]
struct Check {
    id: String,
    severity: String,
    title: String,
    message: String,
}

/// Run `guardrail3 rs validate <path> --format json` and return all check results.
fn validate_project(fixture_name: &str) -> Vec<Check> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = manifest_dir
        .join("tests")
        .join("fixtures")
        .join("adversarial-configs")
        .join(fixture_name);

    assert!(
        fixture_path.exists(),
        "fixture not found: {}",
        fixture_path.display()
    );

    // Find the binary — workspace target dir is at workspace root (two levels up from crate)
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .unwrap_or(&manifest_dir);
    let bin = workspace_root
        .join("target")
        .join("release")
        .join("guardrail3");
    let bin = if bin.exists() {
        bin
    } else {
        workspace_root
            .join("target")
            .join("debug")
            .join("guardrail3")
    };

    assert!(
        bin.exists(),
        "guardrail3 binary not found at {}. Run `cargo build` first.",
        bin.display()
    );

    let output = Command::new(&bin)
        .args(["rs", "validate"])
        .arg(&fixture_path)
        .arg("--format")
        .arg("json")
        .arg("--inventory")
        .output()
        .expect("failed to run guardrail3");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("guardrail3 output is not valid JSON");

    let mut checks = Vec::new();
    if let Some(sections) = json.get("sections").and_then(|s| s.as_array()) {
        for section in sections {
            if let Some(results) = section.get("results").and_then(|r| r.as_array()) {
                for r in results {
                    checks.push(Check {
                        id: r
                            .get("id")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_owned(),
                        severity: r
                            .get("severity")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_owned(),
                        title: r
                            .get("title")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_owned(),
                        message: r
                            .get("message")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_owned(),
                    });
                }
            }
        }
    }

    checks
}

/// Assert that at least one check with the given ID and severity exists.
fn assert_has_check(checks: &[Check], id: &str, severity: &str) {
    let found = checks.iter().any(|c| c.id == id && c.severity == severity);

    if !found {
        let matching_id: Vec<&Check> = checks.iter().filter(|c| c.id == id).collect();
        if matching_id.is_empty() {
            panic!(
                "BUG: check {id} never fired (expected severity={severity}). \
                 Total checks: {}",
                checks.len()
            );
        }
        panic!(
            "BUG: check {id} fired but not with severity={severity}. \
             Found: {:?}",
            matching_id
                .iter()
                .map(|c| format!("{}:{}", c.severity, c.title))
                .collect::<Vec<_>>()
        );
    }
}

/// Assert that at least one check with the given ID, severity, and a message
/// containing the substring exists.
fn assert_has_check_containing(checks: &[Check], id: &str, severity: &str, msg_substr: &str) {
    let found = checks.iter().any(|c| {
        c.id == id
            && c.severity == severity
            && (c.message.contains(msg_substr) || c.title.contains(msg_substr))
    });

    if !found {
        let matching_id: Vec<&Check> = checks.iter().filter(|c| c.id == id).collect();
        panic!(
            "BUG: check {id} with severity={severity} containing \"{msg_substr}\" not found. \
             Matching {id} checks: {:?}",
            matching_id
                .iter()
                .map(|c| format!("{}:{}: {}", c.severity, c.title, c.message))
                .collect::<Vec<_>>()
        );
    }
}

/// Assert that NO check with the given ID fires at error level.
fn assert_no_error(checks: &[Check], id: &str) {
    let errors: Vec<&Check> = checks
        .iter()
        .filter(|c| c.id == id && c.severity == "error")
        .collect();

    if !errors.is_empty() {
        panic!(
            "Expected no {id} errors but found {}: {:?}",
            errors.len(),
            errors
                .iter()
                .map(|c| format!("{}: {}", c.title, c.message))
                .collect::<Vec<_>>()
        );
    }
}

// ---------------------------------------------------------------------------
// Missing config files
// ---------------------------------------------------------------------------

#[test]
fn r1_missing_clippy_toml_detected() {
    let checks = validate_project("no-clippy-toml");
    assert_has_check(&checks, "R1", "error");
}

#[test]
fn r8_missing_deny_toml_detected() {
    let checks = validate_project("no-deny-toml");
    assert_has_check(&checks, "R8", "error");
}

#[test]
fn r21_missing_rustfmt_toml_detected() {
    let checks = validate_project("no-rustfmt-toml");
    assert_has_check(&checks, "R21", "error");
}

#[test]
fn r24_missing_toolchain_detected() {
    let checks = validate_project("no-toolchain");
    assert_has_check(&checks, "R24", "error");
}

#[test]
fn r49_missing_claude_md_detected() {
    let checks = validate_project("no-claude-md");
    // CLAUDE.md missing fires as warn (not error) — acceptable, just verify it fires
    assert_has_check(&checks, "R49", "warn");
}

// ---------------------------------------------------------------------------
// Incomplete configs
// ---------------------------------------------------------------------------

#[test]
fn r4_incomplete_clippy_method_bans_detected() {
    let checks = validate_project("incomplete-clippy");
    // Should detect missing method bans (we only included std::env::var)
    assert_has_check(&checks, "R4", "error");
    // Verify a specific missing ban
    assert_has_check_containing(&checks, "R4", "error", "std::fs::read_to_string");
}

#[test]
fn r5_incomplete_clippy_type_bans_detected() {
    let checks = validate_project("incomplete-clippy");
    // Should detect missing type bans (we only included HashMap)
    assert_has_check(&checks, "R5", "error");
    // Verify a specific missing ban
    assert_has_check_containing(&checks, "R5", "error", "std::sync::Mutex");
}

#[test]
fn r12_incomplete_deny_bans_detected() {
    let checks = validate_project("incomplete-deny-bans");
    // Should detect missing crate bans (we only included openssl)
    assert_has_check(&checks, "R12", "error");
    // Verify specific missing bans
    assert_has_check_containing(&checks, "R12", "error", "chrono");
    assert_has_check_containing(&checks, "R12", "error", "anyhow");
}

#[test]
fn r14_missing_deny_licenses_detected() {
    let checks = validate_project("missing-deny-licenses");
    assert_has_check(&checks, "R14", "error");
}

#[test]
fn r16_missing_deny_sources_detected() {
    let checks = validate_project("missing-deny-sources");
    assert_has_check(&checks, "R16", "error");
}

#[test]
fn r26_missing_cargo_lints_detected() {
    let checks = validate_project("missing-cargo-lints");
    assert_has_check(&checks, "R26", "error");
}

// ---------------------------------------------------------------------------
// Subtle violations
// ---------------------------------------------------------------------------

#[test]
fn r27_relaxed_clippy_lint_detected() {
    let checks = validate_project("relaxed-clippy-lint");
    // unwrap_used = "warn" instead of "deny" should trigger R27 error
    assert_has_check(&checks, "R27", "error");
    assert_has_check_containing(&checks, "R27", "error", "unwrap_used");
}

#[test]
fn r29_no_lint_inheritance_detected() {
    let checks = validate_project("no-lint-inheritance");
    assert_has_check(&checks, "R29", "error");
    assert_has_check_containing(&checks, "R29", "error", "subcrate");
}

// ---------------------------------------------------------------------------
// Negative tests: fixtures that should NOT trigger certain errors
// ---------------------------------------------------------------------------

#[test]
fn r12_complete_deny_bans_no_false_positive() {
    // incomplete-deny-bans has a valid [bans] section with at least one entry
    // R8 (deny.toml exists) should NOT be an error
    let checks = validate_project("incomplete-deny-bans");
    assert_no_error(&checks, "R8");
}

#[test]
fn r14_present_licenses_no_false_positive() {
    // incomplete-deny-bans has a complete [licenses] section
    // R14 should NOT be an error
    let checks = validate_project("incomplete-deny-bans");
    assert_no_error(&checks, "R14");
}

#[test]
fn r16_present_sources_no_false_positive() {
    // incomplete-deny-bans has a complete [sources] section
    // R16 should NOT be an error
    let checks = validate_project("incomplete-deny-bans");
    assert_no_error(&checks, "R16");
}

#[test]
fn r26_present_workspace_lints_no_false_positive() {
    // relaxed-clippy-lint has [workspace.lints.rust] and [workspace.lints.clippy]
    // R26 completeness errors should be limited (not ALL missing)
    let checks = validate_project("relaxed-clippy-lint");
    // R26 should have mostly info (correct), not all errors
    let r26_info = checks
        .iter()
        .filter(|c| c.id == "R26" && c.severity == "info")
        .count();
    assert!(
        r26_info > 20,
        "Expected many R26 info (correct lint) results, got {r26_info}"
    );
}
